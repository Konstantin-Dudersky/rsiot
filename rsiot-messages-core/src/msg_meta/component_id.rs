use serde::{Deserialize, Serialize};

use super::ServiceId;

/// Идентификатор сервиса
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ComponentId(Option<String>);

impl ComponentId {
    pub fn new(service_id: &ServiceId, prefix: &str) -> Self {
        let id = format!("{service_id}:{prefix}");
        Self(Some(id))
    }
}

impl PartialEq for ComponentId {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_none() {
            return false;
        }
        self.0 == other.0
    }
}
