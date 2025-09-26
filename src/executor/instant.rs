use std::time::Duration;

use serde::{Deserialize, Serialize, de::Visitor};

/// Монолитно увеличивающееся время
///
/// Обертка над `std::time::Instant`. Может работать на платформе WebAssembly. Реализованы трейты
/// Default, Serialize и Deserialize
#[derive(Clone, Debug, PartialEq)]
pub struct Instant {
    inst: web_time::Instant,
}

impl Instant {
    /// Returns an instant corresponding to “now”.
    pub fn now() -> Self {
        Self {
            inst: web_time::Instant::now(),
        }
    }

    /// Returns the amount of time elapsed from another instant to this one, or zero duration if
    /// that instant is later than this one.
    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.inst.duration_since(earlier.inst)
    }

    /// Returns the amount of time elapsed since this instant.
    pub fn elapsed(&self) -> Duration {
        self.inst.elapsed()
    }
}

impl Default for Instant {
    fn default() -> Self {
        Self {
            inst: web_time::Instant::now(),
        }
    }
}

impl Serialize for Instant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u128(self.inst.elapsed().as_millis())
    }
}

struct InstantVisitor;

impl Visitor<'_> for InstantVisitor {
    type Value = Instant;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer for TimeInstant")
    }

    fn visit_u128<E>(self, _v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Instant::now())
    }
}

impl<'de> Deserialize<'de> for Instant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u128(InstantVisitor)
    }
}
