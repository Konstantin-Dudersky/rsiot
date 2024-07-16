use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::library::drives;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    pub m1: drives::motor::FB,
    pub v1: drives::valve_analog::FB,
    pub valve: drives::valve::FB,
}
