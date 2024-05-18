use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct WsEvent {
    pub q_length: u32
}

impl WsEvent {
    pub fn new(q_length: u32) -> Self {
        WsEvent { q_length }
    }
}