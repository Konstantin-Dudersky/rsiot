use serde::{Deserialize, Serialize};

use crate::msg_meta::{ComponentId, Timestamp};

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
    pub cmp_source: ComponentId,
    pub cmp_process: ComponentId,
}

impl<TValue> MsgContent<TValue>
where
    TValue: IMsgContentValue + Default,
{
    /// Новое значение, метка времени - now()
    pub fn new(value: TValue) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    /// Новое значение с меткой времени
    pub fn new_with_ts(value: TValue, ts: Timestamp) -> Self {
        Self {
            value,
            ts,
            ..Default::default()
        }
    }

    pub fn cmp_set(&mut self, component_id: ComponentId) {
        self.cmp_source
    }
}
