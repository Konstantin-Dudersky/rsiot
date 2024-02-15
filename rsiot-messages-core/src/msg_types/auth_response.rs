use serde::{Deserialize, Serialize};

use crate::msg_meta::ExecutorId;

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponse {
    service_id: ExecutorId,
    answer: bool,
}
