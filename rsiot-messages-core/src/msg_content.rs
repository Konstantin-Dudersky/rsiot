use serde::{Deserialize, Serialize};

use crate::msg_meta::{ServiceId, Timestamp};

use super::msg_content_value::IMsgContentValue;
/// Тип "Значение"
///
/// Содержит значение типа обобщенного типа `T`, с меткой времени
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsgContent<TValue>
where
    TValue: IMsgContentValue,
{
    pub value: TValue,
    pub ts: Timestamp,
    pub source: ServiceId,
}

impl<TValue> MsgContent<TValue>
where
    TValue: IMsgContentValue,
{
    /// Новое значение, метка времени - now()
    pub fn new(value: TValue) -> Self {
        Self {
            value,
            ts: Timestamp::default(),
            source: ServiceId::default(),
        }
    }

    /// Новое значение с меткой времени
    pub fn new_with_ts(value: TValue, ts: Timestamp) -> Self {
        Self {
            value,
            ts,
            source: ServiceId::default(),
        }
    }
}
