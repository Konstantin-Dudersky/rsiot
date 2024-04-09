use rsiot::message::MsgDataBound;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Gpio0Button(bool),
    Gpio9BootButton(bool),
    SetRelayState(bool),
    HttpbinGet(String),
}
impl MsgDataBound for Custom {}
