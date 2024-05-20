use anyhow::{ Error, anyhow };
use std::str::*;
use std::env::var;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{debug, error, info, instrument, span, warn, Level};
use tracing_subscriber::FmtSubscriber;
use std::net::SocketAddr;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router
};
use tower_http::cors::{Any, CorsLayer};



// mod ws_server; use ws_server::*;
mod data_server;
use shared::types::*;

pub const DEFAULT_LOG_LEVEL: &str = "DEBUG";
pub const CHAN_BOUND: usize = 10;


fn setup_tracing() -> Result<(), Error> {
    let log_level = Level::from_str(var("LOG_LEVEL").unwrap_or(DEFAULT_LOG_LEVEL.to_string()).as_str())?;
    let subscriber = FmtSubscriber::builder()
    .with_max_level(log_level)
    .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let (event_sender, event_receiver) = mpsc::channel(CHAN_BOUND);

//     let _ = task::spawn(async move {
//         ws_server::run(event_receiver).await;
//     });

//     let mut cnt = 0;
//     loop {
//         event_sender.send(WsEvent::new(cnt)).await?;

//         tokio::time::sleep(Duration::from_nanos(1_000_000_000)).await;
//         cnt += 1;
//     }
// }

#[tokio::main]
async fn main() {
    setup_tracing();

    // build our application with a single route
    // let app = Router::new().route("/ws", get(ws_handler));

    let app = Router::new()
        .route("/ws", get(move |ws: WebSocketUpgrade| {
            // Use the cloned ws_list inside the closure
            websocket_handler(ws)
        }));

    // let app = Router::new()
    // .route("/ws", get(ws_handler))
    // .layer(
    //     CorsLayer::new()
    //         .allow_origin(Any)
    //         .allow_methods(Any)
    //         .allow_headers(Any),
    // );

//     var wss = new WebSocketServer({ port: env.PORT, headers: {
//         "Access-Control-Allow-Origin": "*",
//         "Access-Control-Allow-Headers": "http://localhost:3000",
//         "Access-Control-Allow-Methods": "PUT, GET, POST, DELETE, OPTIONS"
// } });

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn websocket_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    debug!("upgrading to ws connection...");
    ws.on_upgrade(|socket| data_server::handle_socket(socket))
}

// async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
//     debug!("upgrading to ws connection...");
//     ws.on_upgrade(handle_socket)
// }