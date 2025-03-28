use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    InjPeriodic(InjPeriodic),
    Filesystem(Filesystem),
}

impl MsgDataBound for Msg {}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum InjPeriodic {
    Increase,
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Filesystem {
    Counter(u64),
}
