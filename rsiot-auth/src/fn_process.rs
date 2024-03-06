use std::time::Duration;

use rsiot_component_core::CmpInOut;
use rsiot_messages_core::{system_messages::*, *};
use tokio::time::sleep;
use tracing::{info, warn};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::{Config, ConfigStore, ConfigStoreItem, Error};

pub async fn fn_process<TMsg>(config: Config, in_out: CmpInOut<TMsg>) -> crate::Result<()>
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

async fn task_main<TMsg>(config: Config, mut in_out: CmpInOut<TMsg>) -> crate::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    while let Ok(msg) = in_out.recv_input().await {
        let msg_response = match msg.data {
            MsgData::System(data) => match data {
                System::AuthRequestByLogin(value) => {
                    process_login_request(value, &config, msg.trace).await?
                }
                _ => continue,
            },
            _ => continue,
        };
        in_out
            .send_output(msg_response)
            .await
            .map_err(Error::CmpOutput)?;
    }
    Ok(())
}

async fn process_login_request<TMsg>(
    login_request: AuthRequestByLogin,
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
            let value = AuthResponseErr { error };
            let msg = message_new!("System-AuthResponseErr::value");
            return Ok(msg);
        }
    };

    // Пароль не подходит
    if valid_password.password != login_request.password {
        let error = "Wrong password".to_string();
        let value = AuthResponseErr { error };
        let msg = message_new!("System-AuthResponseErr::value");
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
        perm: valid_password.role,
        trace_ids,
        login: login_request.login,
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
