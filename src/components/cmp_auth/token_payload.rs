use serde::{Deserialize, Serialize};

use crate::message::AuthPermissions;

#[derive(Deserialize, Serialize)]
pub struct TokenPayload {
    pub login: String,
    pub role: AuthPermissions,
}
