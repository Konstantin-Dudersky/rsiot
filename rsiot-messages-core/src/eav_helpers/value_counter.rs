use chrono::{DateTime, FixedOffset};

use crate::eav;

/// Счетчик с нарастающим итогом
pub struct ValueCounter {
    pub ts: DateTime<FixedOffset>,
    pub entity: String,
    pub attr: String,
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
