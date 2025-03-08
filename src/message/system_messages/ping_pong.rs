use serde::{Deserialize, Serialize};

use crate::message::MsgKey;

/// Проверка связи - запрос партнера по коммуникации
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ping {
    /// Порядковый номер запроса
    pub count: u32,
}

impl MsgKey for Ping {
    fn key(&self) -> String {
        "".to_string()
    }
}

/// Проверка связи - ответ от партнера по коммуникации
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pong {
    /// Порядковый номер запроса
    pub count: u32,
}

impl MsgKey for Pong {
    fn key(&self) -> String {
        "".to_string()
    }
}
