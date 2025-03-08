use serde::{Deserialize, Serialize};

use rsiot::message::{example_service::*, MsgDataBound, MsgKey};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Data {
    TestMessage(i32),
}

impl MsgDataBound for Data {
    type TService = Service;
}
