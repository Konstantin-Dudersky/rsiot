use serde::{Deserialize, Serialize};

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseErr {
    pub error: String,
}
