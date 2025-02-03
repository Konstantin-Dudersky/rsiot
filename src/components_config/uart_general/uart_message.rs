use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UartMessage {
    pub address: u8,
    pub payload: Vec<u8>,
}
