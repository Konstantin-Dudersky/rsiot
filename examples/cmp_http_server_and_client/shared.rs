use rsiot::components_config::http_general::HttpDataBound;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ServerToClient {
    pub counter: f64,
}
impl HttpDataBound for ServerToClient {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ClientToServer {
    #[default]
    NoData,
    SetCounterFromClient(u8),
}
impl HttpDataBound for ClientToServer {}
