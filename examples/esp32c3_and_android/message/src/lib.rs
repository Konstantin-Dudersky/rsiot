use rsiot::message::MsgDataBound;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    BootButton(bool),
}
impl MsgDataBound for Custom {}
