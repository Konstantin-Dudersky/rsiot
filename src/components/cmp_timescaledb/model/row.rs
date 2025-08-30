//! Модель строки в БД

use sqlx::{types::time::OffsetDateTime, FromRow};

use super::agg_type::AggType;

/// Модель строки в БД
#[derive(Debug, FromRow)]
pub struct Row {
    /// Метка времени
    pub time: OffsetDateTime,
    /// Сущность
    pub entity: String,
    /// Атрибут
    pub attr: String,
    /// Значение
    pub value: f64,
    /// Аггрегация, с помощью которой значение было получено
    pub agg: AggType,
    /// Метка времени аггрегации
    pub aggts: Option<OffsetDateTime>,
    /// Массив следующих аггрегаций
    pub aggnext: Vec<AggType>,
}

impl Row {
    /// Создать строку в таблице, только entity, attr и value
    pub fn new_simple(entity: &str, attr: &str, value: f64) -> Self {
        Self {
            time: OffsetDateTime::now_utc(),
            entity: entity.to_string(),
            attr: attr.to_string(),
            value,
            agg: AggType::Curr,
            aggts: None,
            aggnext: vec![],
        }
    }

    /// Создать строку в таблице с указанным временем
    pub fn new_with_ts(entity: &str, attr: &str, value: f64, ts: OffsetDateTime) -> Self {
        Self {
            time: ts,
            entity: entity.to_string(),
            attr: attr.to_string(),
            value,
            agg: AggType::Curr,
            aggts: None,
            aggnext: vec![],
        }
    }
}
