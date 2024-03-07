use serde::{Deserialize, Serialize};

use rsiot_messages_core::AuthPermissions;

#[derive(Deserialize, Serialize)]
pub struct TokenPayload {
    pub login: String,
    pub role: AuthPermissions,
}
