use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    ValueWrite(f64),
    ValueRead(f64),
}

impl MsgDataBound for Msg {}
