use crate::message::{eav, Timestamp};

/// Счетчик с нарастающим итогом
pub struct ValueCounter {
    pub ts: Timestamp,
    pub entity: String,
    pub attr: Option<String>,
    pub value: eav::ValueType,
}

impl From<ValueCounter> for Vec<eav::EavModel> {
    fn from(value: ValueCounter) -> Self {
        let eav_value = eav::EavModel {
            ts: value.ts,
            entity: value.entity,
            attr: value.attr,
            value: value.value,
            agg: eav::AggType::Current,
            aggts: None,
            aggnext: vec![eav::AggType::Inc, eav::AggType::First],
        };
        vec![eav_value]
    }
}
