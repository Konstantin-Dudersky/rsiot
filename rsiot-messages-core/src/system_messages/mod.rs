mod auth_check_token;
mod auth_login_request;
mod auth_login_response;

pub use auth_login_request::AuthLoginRequest;
pub use auth_login_response::AuthLoginResponse;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum System {
    AuthLoginRequest(AuthLoginRequest),
    AuthLoginResponse(AuthLoginResponse),
}
