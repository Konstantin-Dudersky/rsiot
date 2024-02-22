use rsiot_messages_core::{Deserialize, MsgDataBound, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    OutputValue(u16),
}

impl MsgDataBound for Data {}
