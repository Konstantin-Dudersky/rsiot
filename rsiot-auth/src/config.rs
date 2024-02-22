use rsiot_messages_core::MsgDataBound;

use crate::{types::AuthLoginRequest, AuthLoginResponse};

pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    secret_key: String,
    fn_login_input: fn(&TMsg) -> Option<AuthLoginRequest>,
    fn_login_output: fn(&AuthLoginResponse) -> TMsg,
}
