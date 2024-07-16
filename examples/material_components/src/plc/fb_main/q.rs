use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::library::drives;

/// Область памяти output
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    pub motor_hmi_status: drives::motor::QHmiStatus,
    pub valve_analog_hmi_status: drives::valve_analog::QHmiStatus,
    pub valve_hmi_status: drives::valve::QHmiStatus,
}
