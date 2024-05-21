use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
};
use futures::{sink::SinkExt, stream::StreamExt};
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
use clickhouse::{query, Client, Row};
use serde::Deserialize;


#[derive(Debug, Deserialize, Row)]
struct EventOutcome {
    decision_timestamp: i64,
    running_avg_snooze: f64,
    running_avg_remove: f64,
    running_avg_unsubscribe: f64,
    running_avg_send_mail_action: f64,
}

        // toDateTime64(interval_start, 9) AS decision_timestamp,  -- Cast interval_start to DateTime64(9)
fn mk_query() -> String {
    format!("WITH events_per_minute AS (
        SELECT
            toStartOfInterval(decision_timestamp, INTERVAL 1 MINUTE) AS interval_start,
            countIf(action_type = 'snooze') AS count_snooze,
            countIf(action_type = 'remove') AS count_remove,
            countIf(action_type = 'unsubscribe') AS count_unsubscribe,
            countIf(action_type = 'send_mail_action') AS count_send_mail_action
        FROM
            event_outcomes
        GROUP BY
            interval_start
    ),
    running_averages AS (
        SELECT
            toDateTime64(interval_start, 9) AS interval_start,  -- Cast interval_start to DateTime64(9)
            avg(count_snooze) OVER (ORDER BY interval_start ROWS BETWEEN 60 PRECEDING AND CURRENT ROW) AS avg_snooze,
            avg(count_remove) OVER (ORDER BY interval_start ROWS BETWEEN 60 PRECEDING AND CURRENT ROW) AS avg_remove,
            avg(count_unsubscribe) OVER (ORDER BY interval_start ROWS BETWEEN 60 PRECEDING AND CURRENT ROW) AS avg_unsubscribe,
            avg(count_send_mail_action) OVER (ORDER BY interval_start ROWS BETWEEN 60 PRECEDING AND CURRENT ROW) AS avg_send_mail_action
        FROM
            events_per_minute
    )
    SELECT
        interval_start AS decision_timestamp,
        avg_snooze AS running_avg_snooze,
        avg_remove AS running_avg_remove,
        avg_unsubscribe AS running_avg_unsubscribe,
        avg_send_mail_action AS running_avg_send_mail_action
    FROM
        running_averages
    ORDER BY
        decision_timestamp")
}

pub async fn handle_socket(mut socket: WebSocket, client: Client) {
    debug!("handling ws connection...");
    while let Some(Ok(Message::Text(_))) = socket.next().await {

        let query = mk_query();

        match client.query(&query).fetch_all::<EventOutcome>().await {
            Ok(rows) => {
                // Create a Polars DataFrame
                let timestamps: Vec<i64> = rows.iter().map(|r| r.decision_timestamp).collect();
                let avg_snooze: Vec<f64> = rows.iter().map(|r| r.running_avg_snooze).collect();
                let avg_remove: Vec<f64> = rows.iter().map(|r| r.running_avg_remove).collect();
                let avg_unsubscribe: Vec<f64> = rows.iter().map(|r| r.running_avg_unsubscribe).collect();
                let avg_send_mail_action: Vec<f64> = rows.iter().map(|r| r.running_avg_send_mail_action).collect();

                let df = DataFrame::new(vec![
                    Series::new("decision_timestamp", &timestamps),
                    Series::new("running_avg_snooze", &avg_snooze),
                    Series::new("running_avg_remove", &avg_remove),
                    Series::new("running_avg_unsubscribe", &avg_unsubscribe),
                    Series::new("running_avg_send_mail_action", &avg_send_mail_action),
                ]).unwrap();

                // Convert Polars DataFrame to Arrow RecordBatch
                let schema = Arc::new(Schema::new(vec![
                    Field::new("decision_timestamp", DataType::Int64, false),
                    Field::new("running_avg_snooze", DataType::Float64, false),
                    Field::new("running_avg_remove", DataType::Float64, false),
                    Field::new("running_avg_unsubscribe", DataType::Float64, false),
                    Field::new("running_avg_send_mail_action", DataType::Float64, false),
                ]));
                let batch = RecordBatch::try_new(schema.clone(), vec![
                    Arc::new(Int64Array::from(timestamps)) as Arc<dyn arrow::array::Array>,
                    Arc::new(Float64Array::from(avg_snooze)) as Arc<dyn arrow::array::Array>,
                    Arc::new(Float64Array::from(avg_remove)) as Arc<dyn arrow::array::Array>,
                    Arc::new(Float64Array::from(avg_unsubscribe)) as Arc<dyn arrow::array::Array>,
                    Arc::new(Float64Array::from(avg_send_mail_action)) as Arc<dyn arrow::array::Array>,
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