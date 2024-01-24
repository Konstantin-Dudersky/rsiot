use serde::{Deserialize, Serialize};

use crate::msg_meta::{ServiceId, Timestamp};

/// Тип "Значение"
///
/// Содержит значение типа обобщенного типа `T`, с меткой времени
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsgContent<T> {
    pub value: T,
    pub ts: Timestamp,
    pub source: ServiceId,
}

impl<T> MsgContent<T> {
    /// Новое значение, метка времени - now()
    pub fn new(value: T) -> Self {
        Self {
            value,
            ts: Timestamp::default(),
            source: ServiceId::default(),
        }
    }

    /// Новое значение с меткой времени
    pub fn new_with_ts(value: T, ts: Timestamp) -> Self {
        Self {
            value,
            ts,
            source: ServiceId::default(),
        }
    }
}
