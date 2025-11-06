use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    AddLineFile1(String),
    AddLineFile2(String),
    EndProcessing,
}

impl MsgDataBound for Msg {}
