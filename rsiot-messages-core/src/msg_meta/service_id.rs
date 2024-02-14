use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Идентификатор сервиса
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ServiceId(String);

impl ServiceId {
    /// Создает новый уникальный идентификатор
    pub fn new(prefix: &str) -> Self {
        let id = Uuid::new_v4();
        let id = format!("{prefix}:{id}");
        Self(id)
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
