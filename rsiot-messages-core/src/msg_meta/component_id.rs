use serde::{Deserialize, Serialize};

use super::ServiceId;

/// Идентификатор сервиса
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ComponentId(String);

impl ComponentId {
    pub fn new(service_id: &ServiceId, prefix: &str) -> Self {
        let id = format!("{service_id}:{prefix}");
        Self(id)
    }
}

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
