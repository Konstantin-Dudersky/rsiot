use serde::{Deserialize, Serialize};

/// Роли для доступа в системе
#[derive(Deserialize, Serialize)]
pub enum AuthRoles {
    /// Без ограничений
    NoAccess,
    Monitoring,
    Operatoration,
    Admin,
}

#[derive(Deserialize, Serialize)]
pub struct AuthTokenPayload {
    pub role: AuthRoles,
}
