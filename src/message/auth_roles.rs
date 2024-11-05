use serde::{Deserialize, Serialize};

/// Роли для доступа в системе
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialOrd, PartialEq, Serialize)]
pub enum AuthPermissions {
    /// Без ограничений
    NoAccess,

    /// Только просмотр
    Monitoring,

    /// Управление
    Operatoration,

    /// Администрирование
    Admin,

    /// Полный доступ
    #[default]
    FullAccess,
}
