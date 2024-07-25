use serde::{Deserialize, Serialize};

/// Уровень события
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum EventSeverity {
    #[default]
    Info,
    Warning,
    Error,
}
