mod auth_request_by_login;
mod auth_request_by_token;
mod auth_response_error;
mod auth_response_ok;

pub use auth_request_by_login::AuthRequestByLogin;
pub use auth_request_by_token::AuthRequestByToken;
pub use auth_response_error::AuthResponseErr;
pub use auth_response_ok::AuthResponseOk;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum System {
    AuthRequestByLogin(AuthRequestByLogin),
    AuthRequestByToken(AuthRequestByToken),
    AuthResponseErr(AuthResponseErr),
    AuthResponseOk(AuthResponseOk),
}
