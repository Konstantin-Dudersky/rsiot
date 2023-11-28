//! Представление всех сообщений в унифицированную модель.
//!
//! За основу взята модель [EAV](https://en.wikipedia.org/wiki/Entity%E2%80%93attribute%E2%80%93value_model?useskin=vector)
//! , с добавлениями

use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, Default)]
pub enum EavAggType {
    #[default]
    Curr,
    First,
    Inc,
    Sum,
    Mean,
    Min,
    Max,
}

#[derive(Debug, Clone, Default)]
pub struct Eav {
    /// метка времени
    pub ts: DateTime<FixedOffset>,
    pub entity: String,
    pub attr: String,
    pub value: EavValueType,
    pub agg: EavAggType,
    pub aggts: Option<DateTime<FixedOffset>>,
    pub aggnext: Vec<EavAggType>,
}

#[derive(Debug, Clone)]
pub enum EavValueType {
    F64(f64),
}

impl Default for EavValueType {
    fn default() -> Self {
        Self::F64(0.0)
    }
}

pub trait IntoEav {
    fn into_eav(&self) -> Vec<Eav>;
}

#[cfg(test)]
mod tests {
    use super::*;

    enum TestMessage {
        Variant1(f64),
        Variant2(i32),
    }

    impl IntoEav for TestMessage {
        fn into_eav(&self) -> Vec<Eav> {
            match self {
                TestMessage::Variant1(val) => todo!(),
                TestMessage::Variant2(val) => todo!(),
            }
            vec![]
        }
    }

    #[test]
    fn test1() {}
}
