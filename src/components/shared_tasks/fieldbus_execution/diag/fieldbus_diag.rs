use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{DeviceDiag, StoreDeviceDiag, StoreFieldbusDiag};

/// Диагностика работы шины
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FieldbusDiag {
    /// Количество успешных запросов
    pub response_ok_count: usize,

    /// Количество неуспешных запросов
    pub response_err_count: usize,

    /// Использование шины. 1.0 - 100%
    pub usage: f32,

    /// Состояния устройств на шине
    pub devices_diag: HashMap<String, DeviceDiag>,
}
