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
    extract::Extension,
    response::IntoResponse,
    routing::get,
    Router,
};
// use tower_http::cors::{Any, CorsLayer};
use clickhouse::{Client, Row};



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

fn get_ckz_client() -> Client {
    let url = var("CKZ_URL").unwrap_or("http://localhost:8123".to_string());
    let user = var("CKZ_USER").unwrap_or("default".to_string());
    let password = var("CKZ_PASS").unwrap_or("123".to_string());
    // let db = var("CKZ_DB").unwrap_or("emerald_logs".to_string());
    
    Client::default()
        .with_url(&url)
        .with_user(&user)
        .with_password(&password)
        // .with_database(&db)
}

#[tokio::main]
async fn main() {
    _ = setup_tracing();

    let client = get_ckz_client();

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(client));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| data_server::handle_socket(socket, client))
}