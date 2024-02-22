use crate::{eav, Timestamp};

/// Команда или событие
pub struct Command {
    pub ts: Timestamp,
    pub entity: String,
    pub attr: Option<String>,
}

impl From<Command> for Vec<eav::EavModel> {
    fn from(value: Command) -> Self {
        let eav_value = eav::EavModel {
            ts: value.ts,
            entity: value.entity,
            attr: value.attr,
            value: eav::ValueType::bool(true),
            agg: eav::AggType::Current,
            aggts: None,
            aggnext: vec![eav::AggType::Count, eav::AggType::First],
        };
        vec![eav_value]
    }
}
