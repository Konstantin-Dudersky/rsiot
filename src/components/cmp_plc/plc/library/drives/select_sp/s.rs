use serde::{Deserialize, Serialize};

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// Активно задание из plc
    pub sp_plc_act: bool,

    /// Задание из hmi
    pub sp_hmi: f64,
}
