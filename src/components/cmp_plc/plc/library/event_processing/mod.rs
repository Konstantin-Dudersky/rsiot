//! Обработка событий

pub mod collect_events;
mod define_max_severity;
pub mod event;
mod event_severity;

pub use define_max_severity::define_max_severity;
pub use event_severity::EventSeverity;
