use crate::msg_meta::Timestamp;

use super::{AggType, ValueType};

/// Представление значения сообщения в виде модели EAV
#[derive(Debug, Clone, Default)]
pub struct EavModel {
    /// Метка времени
    pub ts: Timestamp,
    /// `Entity`
    pub entity: String,
    /// `Attribute`
    pub attr: Option<String>,
    /// `Value`
    pub value: ValueType,
    /// Значение `Value` было получено с помощью данного типа аггрегации
    pub agg: AggType,
    /// Метка времени, к которой относится данная аггрегация
    pub aggts: Option<Timestamp>,
    /// Типы аггрегации, которые должны применяться на следующем этапе
    pub aggnext: Vec<AggType>,
}
