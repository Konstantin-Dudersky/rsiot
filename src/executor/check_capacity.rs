use tokio::sync::mpsc;
use tracing::warn;

/// Проверка емкости канала
pub trait CheckCapacity {
    /// Проверить емкость канала и выдать предупреждение, если канал заполнен
    ///
    /// Возвращает true, если емкость меньше заданного уровня
    fn check_capacity(&self, threshold: f64, name: &str) -> &Self;
}

impl<T> CheckCapacity for mpsc::Receiver<T> {
    fn check_capacity(&self, threshold: f64, name: &str) -> &Self {
        let capacity = self.capacity();
        let max_capacity = self.max_capacity();
        check_capacity(threshold, name, capacity, max_capacity);
        self
    }
}

impl<T> CheckCapacity for mpsc::Sender<T> {
    fn check_capacity(&self, threshold: f64, name: &str) -> &Self {
        let capacity = self.capacity();
        let max_capacity = self.max_capacity();
        check_capacity(threshold, name, capacity, max_capacity);
        self
    }
}

fn check_capacity(threshold: f64, name: &str, capacity: usize, max_capacity: usize) {
    let free_percent = (capacity as f64) / (max_capacity as f64);

    let warning = free_percent < threshold;
    if warning {
        warn!(
            "Channel capacity too low: {}; channel: {}",
            free_percent, name
        );
    }
}
