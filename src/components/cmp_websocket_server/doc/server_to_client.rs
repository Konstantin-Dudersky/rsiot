use crate::components_config::websocket_general::WebsocketMessage;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Clone, Debug, IntoStaticStr, Deserialize, Serialize)]
pub enum ServerToClient {
    ServerCounter(u32),
}
impl WebsocketMessage for ServerToClient {}
