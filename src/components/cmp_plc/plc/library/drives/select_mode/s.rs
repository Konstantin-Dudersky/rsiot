use serde::{Deserialize, Serialize};

use super::QMode;

/// Область памяти stat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S {
    /// режим работы
    pub mode: QMode,
}
