use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::library::drives;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    pub motor: drives::motor::FB,
    pub valve_analog: drives::valve_analog::FB,
    pub valve: drives::valve::FB,
}
