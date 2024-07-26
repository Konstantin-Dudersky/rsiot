use serde::{Deserialize, Serialize};

/// Уровень события
#[allow(missing_docs)]
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum EventSeverity {
    #[default]
    Info,
    Warning,
    Error,
}
