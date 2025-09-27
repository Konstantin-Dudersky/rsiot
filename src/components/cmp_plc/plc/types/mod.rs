//! Типы данных в ПЛК

mod resettable;

/// Промежуток времени
pub type TimeDuration = std::time::Duration;

pub use resettable::Resettable;
