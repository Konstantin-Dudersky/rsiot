use serde::{Deserialize, Serialize};

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseError {
    pub error: String,
}
