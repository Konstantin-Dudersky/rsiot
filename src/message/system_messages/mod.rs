//! Типы системных сообщений

mod auth_request_by_login;
mod auth_request_by_token;
mod auth_response_error;
mod auth_response_ok;
mod ping_pong;

pub use auth_request_by_login::AuthRequestByLogin;
pub use auth_request_by_token::AuthRequestByToken;
pub use auth_response_error::AuthResponseErr;
pub use auth_response_ok::AuthResponseOk;
pub use ping_pong::{Ping, Pong};

use serde::{Deserialize, Serialize};

use super::{Message, MsgData, MsgDataBound};

/// Типы системных сообщений
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum System {
    /// Запрос авторизации по логину и паролю
    AuthRequestByLogin(AuthRequestByLogin),

    /// Запрос авторизации по токену
    AuthRequestByToken(AuthRequestByToken),

    /// Отказ в авторизации
    AuthResponseErr(AuthResponseErr),

    /// Подтверждение авторизации
    AuthResponseOk(AuthResponseOk),

    /// Проверка связи - запрос партнера по коммуникации
    Ping(Ping),

    /// Проверка связи - ответ от партнера по коммуникации
    Pong(Pong),

    /// Для ESP - wifi подключен
    EspWifiConnected,
}
