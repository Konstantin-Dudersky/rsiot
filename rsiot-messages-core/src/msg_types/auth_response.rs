use serde::{Deserialize, Serialize};

use crate::msg_meta::ServiceId;

/// Ответ на запрос на регистрацию
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponse {
    service_id: ServiceId,
    answer: bool,
}
