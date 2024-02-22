use serde::{Deserialize, Serialize};

use rsiot_messages_core::MsgDataBound;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    TestMessage(i32),
}

impl MsgDataBound for Data {}
