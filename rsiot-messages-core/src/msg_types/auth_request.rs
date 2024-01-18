use serde::{Deserialize, Serialize};

use super::{ServiceId, Timestamp};

/// Запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthRequest {
    service_id: ServiceId,
    login: String,
    password: String,
    ts: Timestamp,
}
