use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::library::drives;

/// Входная структура
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    pub m1_command: drives::motor::IHmiCommand,
}
