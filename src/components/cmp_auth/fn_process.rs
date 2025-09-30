use std::time::Duration;

use tokio::time::sleep;
use tracing::{info, warn};

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

use crate::{
    executor::MsgBusLinker,
    message::{system_messages::*, *},
};

use super::{Config, ConfigStore, ConfigStoreLocalItem, Error, token_payload::TokenPayload};

pub async fn fn_process<TMsg>(config: Config, in_out: MsgBusLinker<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    loop {
        let result = task_main(config.clone(), in_out.clone()).await;
        warn!("Component error: {:?}", result);
        info!("Restart");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMsg>(config: Config, mut in_out: MsgBusLinker<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    while let Ok(msg) = in_out.recv_input().await {
        let msg_response = match msg.data {
            MsgData::System(data) => match data {
                System::AuthRequestByLogin(value) => {
                    process_request_by_login(value, &config, msg.trace.clone()).await
                }
                System::AuthRequestByToken(value) => {
                    process_request_by_token(value, &config, msg.trace.clone()).await
                }
                _ => continue,
            },
            _ => continue,
        };
        let msg = match msg_response {
            Ok(msg) => {
                info!("Success login: {:?}", msg);
                msg
            }
            Err(err) => {
                warn!("Wrong login attempt: {}", err);
                let trace_ids = msg.trace.get_ids();
                let value = AuthResponseErr {
                    error: err.to_string(),
                    trace_ids,
                };
                Message::new(MsgData::System(System::AuthResponseErr(value)))
            }
        };
        in_out.send_output(msg).await.map_err(Error::CmpOutput)?;
    }
    Ok(())
}

/// Обработка запроса по токену
async fn process_request_by_token<TMsg>(
    request_by_login: AuthRequestByToken,
    config: &Config,
    msg_trace: MsgTrace,
) -> super::Result<Message<TMsg>>
where
    TMsg: MsgDataBound,
{
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.secret_key.as_bytes())?;
    let claims: TokenPayload = request_by_login.token.verify_with_key(&key)?;

    let trace_ids = msg_trace.get_ids();
    let value = AuthResponseOk {
        token: request_by_login.token,
        perm: claims.role,
        trace_ids,
        login: claims.login,
    };
    let msg = Message::new(MsgData::System(System::AuthResponseOk(value)));
    Ok(msg)
}

/// Обработка запроса по логину-паролю
async fn process_request_by_login<TMsg>(
    request_by_login: AuthRequestByLogin,
    config: &Config,
    msg_trace: MsgTrace,
) -> super::Result<Message<TMsg>>
where
    TMsg: MsgDataBound,
{
    info!("Request: {request_by_login:?}");
    let valid_password = get_credentials(&request_by_login.login, config).await?;

    // Пользователь не найден
    let valid_password = valid_password.ok_or(Error::ProcessRequest("Unknown user".into()))?;

    // Пароль не подходит
    if valid_password.password != request_by_login.password {
        return Err(Error::ProcessRequest("Wrong password".into()));
    }

    // Генерируем jwt
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.secret_key.as_bytes())?;
    let claims = TokenPayload {
        login: request_by_login.login.clone(),
        role: valid_password.role,
    };
    let token = claims.sign_with_key(&key)?;

    let trace_ids = msg_trace.get_ids();

    let value = AuthResponseOk {
        token,
        perm: valid_password.role,
        trace_ids,
        login: request_by_login.login,
    };
    let msg = Message::new(MsgData::System(System::AuthResponseOk(value)));

    Ok(msg)
}

async fn get_credentials(
    login: &str,
    config: &Config,
) -> super::Result<Option<ConfigStoreLocalItem>> {
    match &config.store {
        ConfigStore::Local(local) => {
            let item = local.iter().find(|e| e.login == login).cloned();
            Ok(item)
        }

        ConfigStore::Surrealdb => todo!(),
    }
}
