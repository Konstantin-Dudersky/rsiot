use serde::{Deserialize, Serialize};

/// Вход в систему по логину и паролю
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthLoginRequest {
    login: String,
    password: String,
}

impl AuthLoginRequest {
    pub fn new(login: &str, password: &str) -> Self {
        Self {
            login: login.to_string(),
            password: password.to_string(),
        }
    }
}
