use crate::{eav, msg_meta::Timestamp};

/// Мгновенное значение
pub struct ValueInstant {
    pub ts: Timestamp,
    pub entity: String,
    pub attr: Option<String>,
    pub value: eav::ValueType,
}

impl From<ValueInstant> for Vec<eav::EavModel> {
    fn from(value: ValueInstant) -> Self {
        let eav_value = eav::EavModel {
            ts: value.ts,
            entity: value.entity,
            attr: value.attr,
            value: value.value,
            agg: eav::AggType::Current,
            aggts: None,
            aggnext: vec![eav::AggType::Min, eav::AggType::Max, eav::AggType::Mean],
        };
        vec![eav_value]
    }
}
