//! Модель строки в БД

use chrono::{DateTime, FixedOffset};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    FromRow, Type,
};

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "agg_type", rename_all = "lowercase")]
pub enum AggType {
    Curr,
    First,
    Inc,
    Sum,
    Mean,
    Min,
    Max,
}

#[derive(Debug, FromRow)]
pub struct Row {
    pub ts: DateTime<FixedOffset>,
    pub entity: String,
    pub attr: String,
    pub value: Option<f64>,
    pub agg: AggType,
    pub aggts: Option<DateTime<FixedOffset>>,
    pub aggnext: Vec<AggType>,
}

impl Row {
    pub fn new(
        ts: DateTime<FixedOffset>,
        entity: &str,
        attr: &str,
        value: f64,
    ) -> Self {
        Self {
            ts,
            entity: entity.to_string(),
            attr: attr.to_string(),
            value: Some(value),
            agg: AggType::Curr,
            aggts: None,
            aggnext: vec![],
        }
    }
}

impl PgHasArrayType for AggType {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("agg_type[]")
    }
}
