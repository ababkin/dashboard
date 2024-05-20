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


pub async fn handle_socket(mut socket: WebSocket) {
    debug!("handling ws connection...");
    while let Some(Ok(Message::Text(_))) = socket.next().await {
        // Create a Polars DataFrame
        let df = df![
            "a" => &[1, 2, 3, 4, 5],
            "b" => &[1.1, 2.2, 3.3, 4.4, 5.5]
        ].unwrap();

        // Convert Polars DataFrame to Arrow RecordBatch
        let schema = Arc::new(Schema::new(vec![
            Field::new("a", DataType::Int64, false),
            Field::new("b", DataType::Float64, false),
        ]));
        let batch = RecordBatch::try_new(schema.clone(), vec![
            Arc::new(Int64Array::from(df.column("a").unwrap().i64().unwrap().to_vec())) as Arc<dyn arrow::array::Array>,
            Arc::new(Float64Array::from(df.column("b").unwrap().f64().unwrap().to_vec())) as Arc<dyn arrow::array::Array>,
        ]).unwrap();

        // Serialize the RecordBatch to Parquet format (for storage or other uses)
        let mut parquet_buffer = Cursor::new(vec![]);
        let props = WriterProperties::builder().build();
        let mut parquet_writer = ArrowWriter::try_new(&mut parquet_buffer, schema.clone(), Some(props)).unwrap();
        parquet_writer.write(&batch).unwrap();
        parquet_writer.close().unwrap();

        // Serialize the RecordBatch to Arrow IPC stream (sent to client)
        let mut ipc_buffer = Cursor::new(vec![]);
        {
            let mut ipc_writer = StreamWriter::try_new(&mut ipc_buffer, &schema).unwrap();
            ipc_writer.write(&batch).unwrap();
            ipc_writer.finish().unwrap();
        }

        // Send the binary data over WebSocket
        let data = ipc_buffer.into_inner();
        if socket.send(Message::Binary(data)).await.is_err() {
            break;
        }
    }
}