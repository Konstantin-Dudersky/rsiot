use tokio::sync::mpsc;
use tracing::warn;

/// Проверка емкости канала
pub trait CheckCapacity {
    /// Проверить емкость канала и выдать предупреждение, если канал заполнен
    ///
    /// Возвращает true, если емкость меньше заданного уровня
    fn check_capacity(&self, threshold: f64, name: &str) -> bool;
}

impl<T> CheckCapacity for mpsc::Receiver<T> {
    fn check_capacity(&self, threshold: f64, name: &str) -> bool {
        let capacity = self.capacity();
        let max_capacity = self.max_capacity();
        let free_percent = (capacity as f64) / (max_capacity as f64);

        let warning = free_percent < threshold;
        if warning {
            warn!(
                "Channel capacity too low: {}; channel: {}",
                free_percent, name
            );
        }

        warning
    }
}
