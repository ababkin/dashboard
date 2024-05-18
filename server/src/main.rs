use anyhow::{ Error, anyhow };
use std::str::*;
use std::env::var;
use tokio::sync::mpsc;
use tokio::task;
use tracing::{debug, error, info, instrument, span, warn, Level};
use tracing_subscriber::FmtSubscriber;
use std::time::Duration;

mod ws_server; use ws_server::*;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (event_sender, event_receiver) = mpsc::channel(CHAN_BOUND);

    let _ = task::spawn(async move {
        ws_server::run(event_receiver).await;
    });

    let mut cnt = 0;
    loop {
        event_sender.send(WsEvent::new(cnt)).await?;

        tokio::time::sleep(Duration::from_nanos(1_000_000_000)).await;
        cnt += 1;
    }
}
