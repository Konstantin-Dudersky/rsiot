use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Идентификатор сервиса
///
/// Переопределяет метод Default - генерируется уникальный идентификатор
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServiceId(Uuid);

impl Default for ServiceId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
