use serde::{Deserialize, Serialize};

/// Проверка связи - запрос партнера по коммуникации
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Ping {
    /// Порядковый номер запроса
    pub count: u32,
}

/// Проверка связи - ответ от партнера по коммуникации
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Pong {
    /// Порядковый номер запроса
    pub count: u32,
}
