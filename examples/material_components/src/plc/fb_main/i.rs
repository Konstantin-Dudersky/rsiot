use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::library::drives;

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    pub motor_hmi_command: drives::motor::IHmiCommand,
    pub valve_analog_hmi_command: drives::valve_analog::IHmiCommand,
    pub valve_hmi_command: drives::valve::IHmiCommand,
}
