use rsiot_messages_core::IMessage;

use crate::{types::AuthLoginRequest, AuthLoginResponse};

pub struct Config<TMsg>
where
    TMsg: IMessage,
{
    secret_key: String,
    fn_login_input: fn(&TMsg) -> Option<AuthLoginRequest>,
    fn_login_output: fn(&AuthLoginResponse) -> TMsg,
}
