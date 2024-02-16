use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Вход в систему по логину и паролю
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthLoginRequest {
    login: String,
    password: String,
    request_id: Uuid,
}

impl AuthLoginRequest {
    pub fn new(login: &str, password: &str) -> Self {
        Self {
            login: login.to_string(),
            password: password.to_string(),
            request_id: Uuid::new_v4(),
        }
    }
}
