use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthLoginResponse {
    request_id: Uuid,
    token: Option<String>,
}
