use rsiot::{
    components_config::websocket_general::WebsocketMessage,
    message::{example_service, MsgDataBound, MsgKey},
};
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

// ServerMessages ----------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum ServerMessages {
    ServerCounter(u32),
    CounterFromClient(u8),
}
impl MsgDataBound for ServerMessages {
    type TService = example_service::Service;
}

// ClientMessages ----------------------------------------------------------------------------------

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum ClientMessages {
    ServerCounter(u32),
    CounterFromClient(u8),
    ConnectionState(bool),
}
impl MsgDataBound for ClientMessages {
    type TService = example_service::Service;
}
