use serde::{Deserialize, Serialize};

use crate::message::MsgKey;

/// Запрос авторизации по токену
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AuthRequestByToken {
    /// Токен
    pub token: String,
}

impl MsgKey for AuthRequestByToken {
    fn key(&self) -> String {
        "".to_string()
    }
}
