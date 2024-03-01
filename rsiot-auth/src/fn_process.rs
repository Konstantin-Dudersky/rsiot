use std::time::Duration;

use rsiot_component_core::{Cache, CmpInput, CmpOutput};
use rsiot_messages_core::{system_messages::*, *};
use tokio::time::sleep;
use tracing::{info, warn};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::{Config, ConfigStore, ConfigStoreItem, Error};

pub async fn fn_process<TMsg>(
    input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    config: Config,
    _cache: Cache<TMsg>,
) -> crate::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    loop {
        let result = task_main(input.clone(), output.clone(), config.clone()).await;
        warn!("Component error: {:?}", result);
        info!("Restart");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMsg>(
    mut input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    config: Config,
) -> crate::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    while let Ok(msg) = input.recv().await {
        let Some(msg) = msg else { continue };
        let msg_response = match msg.data {
            MsgData::System(data) => match data {
                System::AuthLoginRequest(value) => {
                    process_login_request(value, &config, msg.trace).await?
                }
                _ => continue,
            },
            _ => continue,
        };
        output.send(msg_response).await.map_err(Error::CmpOutput)?;
    }
    Ok(())
}

async fn process_login_request<TMsg>(
    login_request: AuthLoginRequest,
    config: &Config,
    msg_trace: MsgTrace,
) -> crate::Result<Message<TMsg>>
where
    TMsg: MsgDataBound,
{
    let valid_password = get_credentials(&login_request.login, config).await?;

    // Пользователь не найден
    let valid_password = match valid_password {
        Some(valid_password) => valid_password,
        None => {
            let error = format!("Unknown user: {}", login_request.login);
            let value = AuthResponseError { error };
            let msg = message_new!("System-AuthResponseError::value");
            return Ok(msg);
        }
    };

    // Пароль не подходит
    if valid_password.password != login_request.password {
        let error = format!("Wrong password");
        let value = AuthResponseError { error };
        let msg = message_new!("System-AuthResponseError::value");
        return Ok(msg);
    }

    // Генерируем jwt
    let key: Hmac<Sha256> = Hmac::new_from_slice(config.secret_key.as_bytes()).unwrap();
    let claims = AuthTokenPayload {
        role: AuthPermissions::Admin,
    };
    let token = claims.sign_with_key(&key).unwrap();

    let trace_ids = msg_trace.get_ids();

    let value = AuthResponseOk {
        token,
        role: valid_password.role,
        trace_ids,
    };
    let msg = message_new!("System-AuthResponseOk::value");
    Ok(msg)
}

async fn get_credentials(login: &str, config: &Config) -> crate::Result<Option<ConfigStoreItem>> {
    match &config.store {
        ConfigStore::Local(local) => {
            let item = local.iter().find(|e| e.login == login).cloned();
            Ok(item)
        }

        ConfigStore::Surrealdb => todo!(),
    }
}
