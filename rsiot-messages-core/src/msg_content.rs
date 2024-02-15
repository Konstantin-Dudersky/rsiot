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
    /// Значение
    pub value: TValue,

    /// Метка времени
    pub ts: Timestamp,

    /// Компонент-источник сообщения
    pub cmp_source: Option<ComponentId>,

    /// Компонент, в котором сообщение было обработано
    pub cmp_process: Option<ComponentId>,
}

impl<TValue> MsgContent<TValue>
where
    TValue: IMsgContentValue,
{
    /// Новое значение, метка времени - now()
    pub fn new(value: TValue) -> Self {
        Self {
            value,
            ts: Default::default(),
            cmp_source: Default::default(),
            cmp_process: Default::default(),
        }
    }

    /// Новое значение с меткой времени
    pub fn new_with_ts(value: TValue, ts: Timestamp) -> Self {
        Self {
            value,
            ts,
            cmp_source: Default::default(),
            cmp_process: Default::default(),
        }
    }

    pub fn cmp_set(&mut self, component_id: &ComponentId) {
        if self.cmp_source.is_none() {
            self.cmp_source = Some(component_id.clone());
        };
        self.cmp_process = Some(component_id.clone());
    }
}
