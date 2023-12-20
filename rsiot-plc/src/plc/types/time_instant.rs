//! Метка времени
//!
//! Обертка над `std::time::Instant`
//! Добавлена поддрежка Default

use std::time::Instant;

use serde::Serialize;

use crate::plc::types;

#[derive(Clone)]
pub struct TimeInstant {
    inst: Instant,
}

impl TimeInstant {
    pub fn now() -> Self {
        Self {
            inst: Instant::now(),
        }
    }

    pub fn duration_since(&self, earlier: Self) -> types::TimeDuration {
        self.inst.duration_since(earlier.inst)
    }

    pub fn elapsed(&self) -> types::TimeDuration {
        self.inst.elapsed()
    }
}

impl Default for TimeInstant {
    fn default() -> Self {
        Self {
            inst: Instant::now(),
        }
    }
}

impl Serialize for TimeInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u128(self.inst.elapsed().as_millis())
    }
}
