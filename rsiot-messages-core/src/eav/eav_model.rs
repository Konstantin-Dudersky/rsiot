use chrono::{DateTime, FixedOffset};

use super::{AggType, ValueType};

/// Представление значения сообщения в виде модели EAV
#[derive(Debug, Clone, Default)]
pub struct EavModel {
    /// Метка времени
    pub ts: DateTime<FixedOffset>,
    /// `Entity`
    pub entity: String,
    /// `Attribute`
    pub attr: String,
    /// `Value`
    pub value: ValueType,
    /// Значение `Value` было получено с помощью данного типа аггрегации
    pub agg: AggType,
    /// Метка времени, к которой относится данная аггрегация
    pub aggts: Option<DateTime<FixedOffset>>,
    /// Типы аггрегации, которые должны применяться на следующем этапе
    pub aggnext: Vec<AggType>,
}
