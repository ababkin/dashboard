use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;
use polars::prelude::*;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use arrow::array::{Float64Array, Int64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::ipc::writer::StreamWriter;
use arrow::record_batch::RecordBatch;
use std::sync::Arc;
use std::io::Cursor;
use tracing::{debug, error, info, instrument, span, warn, Level};
use clickhouse::{Client, Row};
use serde::Deserialize;


#[derive(Debug, Deserialize, Row)]
struct EventOutcome {
    decision_timestamp: i64,
    running_avg_remove: f64,
}

pub async fn handle_socket(mut socket: WebSocket, client: Client) {
    debug!("handling ws connection...");
    while let Some(Ok(Message::Text(_))) = socket.next().await {
        debug!("got invite for data");

        let query = "WITH running_totals AS (
            SELECT
                toStartOfInterval(decision_timestamp, INTERVAL 1 HOUR) AS interval_start,
                countIf(action_type = 'remove') OVER (ORDER BY toStartOfInterval(decision_timestamp, INTERVAL 1 HOUR) ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS running_count_remove,
                count(*) OVER (ORDER BY toStartOfInterval(decision_timestamp, INTERVAL 1 HOUR) ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS running_count_total
            FROM
                event_outcomes
        )
        SELECT
            toDateTime64(interval_start, 9) AS decision_timestamp,  -- Cast interval_start to DateTime64(9)
            running_count_remove / running_count_total AS running_avg_remove
        FROM
            (SELECT 
                interval_start, 
                max(running_count_remove) AS running_count_remove, 
                max(running_count_total) AS running_count_total 
            FROM 
                running_totals 
            GROUP BY 
                interval_start)
        ORDER BY
            decision_timestamp";

        match client.query(query).fetch_all::<EventOutcome>().await {
            Ok(rows) => {
                // Create a Polars DataFrame
                let timestamps: Vec<i64> = rows.iter().map(|r| r.decision_timestamp).collect();
                let averages: Vec<f64> = rows.iter().map(|r| r.running_avg_remove).collect();

                let df = DataFrame::new(vec![
                    Series::new("decision_timestamp", &timestamps),
                    Series::new("running_avg_remove", &averages),
                ]).unwrap();

                // Convert Polars DataFrame to Arrow RecordBatch
                let schema = Arc::new(Schema::new(vec![
                    Field::new("decision_timestamp", DataType::Int64, false),
                    Field::new("running_avg_remove", DataType::Float64, false),
                ]));
                let batch = RecordBatch::try_new(schema.clone(), vec![
                    Arc::new(Int64Array::from(timestamps)) as Arc<dyn arrow::array::Array>,
                    Arc::new(Float64Array::from(averages)) as Arc<dyn arrow::array::Array>,
                ]).unwrap();

                // Serialize the RecordBatch to Arrow IPC stream
                let mut ipc_buffer = Cursor::new(vec![]);
                {
                    let mut ipc_writer = StreamWriter::try_new(&mut ipc_buffer, &schema).unwrap();
                    ipc_writer.write(&batch).unwrap();
                    ipc_writer.finish().unwrap();
                }

                // Send the binary data over WebSocket
                let data = ipc_buffer.into_inner();
                debug!("writing data to ws connection: {:?}", data);
                if socket.send(Message::Binary(data)).await.is_err() {
                    break;
                }
            }
            Err(err) => {
                eprintln!("Query failed: {:?}", err);
                break;
            }
        }
    }
}