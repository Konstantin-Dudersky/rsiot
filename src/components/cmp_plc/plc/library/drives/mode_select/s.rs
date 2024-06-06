use serde::{Deserialize, Serialize};

use super::QMode;

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// режим работы
    pub mode: QMode,
}
