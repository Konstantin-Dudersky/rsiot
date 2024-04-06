use serde::{Deserialize, Serialize};

use rsiot::message::MsgDataBound;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    Counter(i32),
}

impl MsgDataBound for Custom {}
