use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, FromRequest},
    response::Response,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use uuid::Uuid;
use tokio::sync::Mutex;
use tracing::{debug, error, info, instrument, span, warn, Level};

use shared::types::*;


type SharedWebSocketList = Arc<Mutex<Vec<(Uuid, WebSocket)>>>;


pub async fn run(rcv: mpsc::Receiver<WsEvent>) {

    let ws_list = Arc::new(Mutex::new(Vec::new()));

    // Clone ws_list for the WebSocket route handler
    let ws_list_for_handler = ws_list.clone();

    let app = Router::new()
        .route("/ws", get(move |ws: WebSocketUpgrade| {
            // Use the cloned ws_list inside the closure
            websocket_handler(ws, ws_list_for_handler.clone())
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Use the original ws_list for the broadcast loop
    tokio::spawn(broadcast_loop(rcv, ws_list));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}


pub async fn websocket_handler(ws: WebSocketUpgrade, ws_list: SharedWebSocketList) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_socket(socket, ws_list))
}

async fn handle_socket(socket: WebSocket, ws_list: SharedWebSocketList) {

    debug!("got new WS");

    let id = Uuid::new_v4();  // Generate a unique identifier for each connection
    let mut ws_list_lock = ws_list.lock().await;
    ws_list_lock.push((id, socket));
    drop(ws_list_lock);

}

async fn broadcast_loop(mut rcv: mpsc::Receiver<WsEvent>, ws_list: SharedWebSocketList) {
    use leptos_server_signal::ServerSignal;

    // Assuming ServerSignal is meant to be a shared event manager for all WebSockets
    let mut events = ServerSignal::<WsEvent>::new("counter").unwrap();

    while let Some(event) = rcv.recv().await {
        debug!("received event...");
        let mut ws_list_locked = ws_list.lock().await;  // Lock once per event received

        for (_, ws) in ws_list_locked.iter_mut() {

            // Apply the event to each WebSocket through the ServerSignal, if this is the correct usage
            let _result = events.with(ws, |e| *e = event.clone()).await;
            // let _result = events.with(ws, |e| *e = 3).await;
        }
    }

}