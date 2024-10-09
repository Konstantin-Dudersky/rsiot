//! Типы данных в ПЛК

mod resettable;
mod time_instant;

/// Промежуток времени
pub type TimeDuration = std::time::Duration;

/// Метка времени
pub type TimeInstant = time_instant::TimeInstant;

pub use resettable::Resettable;
