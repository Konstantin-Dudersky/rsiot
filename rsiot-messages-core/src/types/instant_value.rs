use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

use crate::{Eav, EavValueType, IntoEav};

use super::Timestamp;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InstantValue<T> {
    pub value: T,
    pub ts: Timestamp,
}

impl<T> InstantValue<T> {
    pub fn new(value: T, ts: Option<Timestamp>) -> Self {
        let ts = unwrap_ts_or_now(ts);
        Self { value, ts }
    }
}

impl<T> IntoEav for InstantValue<T> {
    fn into_eav(&self) -> Vec<crate::eav::Eav> {
        let val = Eav {
            ts: self.ts,
            value: self.value.into(),
            aggnext: 1,
            ..Default::default()
        };
        vec![val]
    }
}

/// Извлекаем метку времени из Option, или возвращаем текущее время
fn unwrap_ts_or_now(ts: Option<Timestamp>) -> Timestamp {
    match ts {
        Some(value) => value,
        None => Utc::now().into(),
    }
}
