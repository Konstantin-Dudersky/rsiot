use serde::{Deserialize, Serialize};

/// Роли для доступа в системе
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AuthPermissions {
    /// Без ограничений
    NoAccess,
    Monitoring,
    Operatoration,
    Admin,
}

#[derive(Deserialize, Serialize)]
pub struct AuthTokenPayload {
    pub role: AuthPermissions,
}
