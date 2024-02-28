use serde::{Deserialize, Serialize};

use crate::system_messages::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MsgData<TCustom> {
    System(System),
    Custom(TCustom),
}
