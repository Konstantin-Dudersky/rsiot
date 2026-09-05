use rsiot::{
    components::shared_tasks::fieldbus_execution::FieldbusDiag,
    message::{MsgDataBound, MsgKey},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    ValueWrite(f64),
    ValueRead(f64),
    Diag(FieldbusDiag),
}

impl MsgDataBound for Msg {}
