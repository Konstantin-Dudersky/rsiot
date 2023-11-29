use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    Type,
};

use rsiot_messages_core::eav;

/// Представление аггрегации в БД
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
    Count,
}

impl PgHasArrayType for AggType {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("agg_type[]")
    }
}

impl From<eav::AggType> for AggType {
    fn from(value: eav::AggType) -> Self {
        match value {
            eav::AggType::Count => Self::Count,
            eav::AggType::Current => Self::Curr,
            eav::AggType::First => Self::First,
            eav::AggType::Inc => Self::Inc,
            eav::AggType::Max => Self::Max,
            eav::AggType::Mean => Self::Mean,
            eav::AggType::Min => Self::Min,
            eav::AggType::Sum => Self::Sum,
        }
    }
}
