use std::time::Duration;

/// Конфигурация периодических запросов
pub struct ConfigPeriodicRequest<TRequest, TBuffer> {
    /// Период вызова
    pub period: Duration,

    /// Функция создания запросов на основе данных из буфера
    pub fn_requests: fn(&TBuffer) -> anyhow::Result<Vec<TRequest>>,
}
