use rsiot::components_config::websocket_general::WebsocketMessage;
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

#[derive(Clone, Debug, IntoStaticStr, Deserialize, Serialize)]
pub enum ServerToClient {
    ServerCounter(u32),
}
impl WebsocketMessage for ServerToClient {}

#[derive(Clone, Debug, IntoStaticStr, Deserialize, Serialize)]
pub enum ClientToServer {
    ClientCounter(u8),
}
impl WebsocketMessage for ClientToServer {}
