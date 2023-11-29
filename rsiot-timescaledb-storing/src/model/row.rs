//! Модель строки в БД

use chrono::{DateTime, FixedOffset};
use sqlx::FromRow;

use rsiot_messages_core::eav;

use super::agg_type::AggType;

/// Модель строки в БД
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

impl From<eav::EavModel> for Row {
    fn from(eav_model: eav::EavModel) -> Self {
        let value = match eav_model.value {
            eav::ValueType::bool(_) => todo!(),
            eav::ValueType::f64(value) => value,
            eav::ValueType::String(_) => todo!(),
            eav::ValueType::u64(value) => value as f64,
        };

        let row = Row {
            ts: eav_model.ts,
            entity: eav_model.entity,
            attr: eav_model.attr,
            value: Some(value),
            agg: eav_model.agg.into(),
            aggts: eav_model.aggts,
            aggnext: eav_model.aggnext.iter().map(|a| a.clone().into()).collect(),
        };

        row
    }
}
