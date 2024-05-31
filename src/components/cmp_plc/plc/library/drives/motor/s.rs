use serde::{Deserialize, Serialize};

/// Область памяти stat
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    /// режим работы
    pub mode: SMode,

    /// Команда на запуск
    pub start: bool,
}

/// Режим работы
#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
pub enum SMode {
    /// Автоматический режим
    Auto,

    /// Местный
    Local,

    /// Ручной
    #[default]
    Manual,

    /// Out of service - выведен из эксплуатации
    Oos,
}
