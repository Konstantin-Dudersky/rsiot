use rsiot::{components::cmp_plc::plc::library::drives, message::*};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    m1_status(drives::motor::QHmiStatus),
    m1_command(drives::motor::IHmiCommand),
}

impl MsgDataBound for Custom {}
