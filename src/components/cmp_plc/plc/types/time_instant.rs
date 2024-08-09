//! Метка времени
//!
//! Обертка над `std::time::Instant`
//! Добавлена поддрежка Default

use std::time::Instant;

use serde::{de::Visitor, Deserialize, Serialize};

use super::TimeDuration;

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

    pub fn duration_since(&self, earlier: Self) -> TimeDuration {
        self.inst.duration_since(earlier.inst)
    }

    pub fn elapsed(&self) -> TimeDuration {
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

struct TimeInstantVisitor;

impl<'de> Visitor<'de> for TimeInstantVisitor {
    type Value = TimeInstant;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer for TimeInstant")
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TimeInstant::now())
    }
}

impl<'de> Deserialize<'de> for TimeInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u128(TimeInstantVisitor)
    }
}
