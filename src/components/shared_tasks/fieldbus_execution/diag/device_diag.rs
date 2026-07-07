use std::{collections::BTreeMap, time::Duration};

use serde::{Deserialize, Serialize};

use super::StoreDeviceDiag;

/// Состояние устройства
#[derive(Default, Deserialize, Clone, Debug, PartialEq, Serialize)]
pub struct DeviceDiag {
    /// Инициализация завершена
    pub init_completed: bool,

    /// Количество успешных ответов
    pub response_ok_count: usize,

    /// Количество ошибочных ответов
    pub response_err_count: usize,

    /// Средняя продолжительность выполнения запроса
    pub avg_request_duration: Duration,

    pub last_errors: BTreeMap<String, String>,
}
