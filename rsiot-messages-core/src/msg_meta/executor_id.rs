use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Идентификатор сервиса
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExecutorId(String);

impl ExecutorId {
    /// Создает новый уникальный идентификатор
    pub fn new(executor_name: &str) -> Self {
        let id = Uuid::new_v4();
        let id = format!("{executor_name}({id})");
        Self(id)
    }
}

impl std::fmt::Display for ExecutorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
