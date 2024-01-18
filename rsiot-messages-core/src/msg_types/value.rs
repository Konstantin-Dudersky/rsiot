use serde::{Deserialize, Serialize};

use super::Timestamp;

/// Тип "Значение"
///
/// Содержит значение типа обобщенного типа `T`, с меткой времени
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Value<T> {
    pub value: T,
    pub ts: Timestamp,
}

impl<T> Value<T> {
    /// Новое значение, метка времени - now()
    pub fn new(value: T) -> Self {
        Self {
            value,
            ts: Timestamp::default(),
        }
    }

    /// Новое значение с меткой времени
    pub fn new_with_ts(value: T, ts: Timestamp) -> Self {
        Self { value, ts }
    }
}
