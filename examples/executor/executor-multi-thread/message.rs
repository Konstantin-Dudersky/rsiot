use serde::{Deserialize, Serialize};

use rsiot::message::{example_service::*, MsgDataBound};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Data {
    TestMessage(i32),
}

impl MsgDataBound for Data {
    type TService = Service;
}
