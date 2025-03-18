use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServerToClient {
    pub counter: f64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ClientToServer {
    #[default]
    NoData,
    SetCounterFromClient(u8),
}
