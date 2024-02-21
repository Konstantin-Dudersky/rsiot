use serde::{Deserialize, Serialize};

use rsiot_messages_core::message_v2::MsgDataBound;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    TestMessage(i32),
}

impl MsgDataBound for Data {}
