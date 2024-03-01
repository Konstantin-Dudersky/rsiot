use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AuthPermissions;

/// Ответ на запрос на регистрацию
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthResponseOk {
    pub token: String,
    pub role: AuthPermissions,
    pub trace_ids: HashSet<Uuid>,
}
