mod auth_check_token;
mod auth_login_request;
mod auth_response_error;
mod auth_response_ok;

pub use auth_login_request::AuthLoginRequest;
pub use auth_response_error::AuthResponseError;
pub use auth_response_ok::AuthResponseOk;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum System {
    AuthLoginRequest(AuthLoginRequest),
    AuthResponseOk(AuthResponseOk),
    AuthResponseError(AuthResponseError),
}
