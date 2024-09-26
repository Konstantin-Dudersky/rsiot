use rsiot::{
    components::cmp_plc::plc::library::drives,
    message::{example_service::Service, *},
};
use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    m1_status(drives::motor::QHmiStatus),
    m1_command(drives::motor::IHmiCommand),

    valve_analog_status(drives::valve_analog::QHmiStatus),
    valve_analog_command(drives::valve_analog::IHmiCommand),

    valve_hmi_command(drives::valve::IHmiCommand),
    valve_hmi_status(drives::valve::QHmiStatus),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
