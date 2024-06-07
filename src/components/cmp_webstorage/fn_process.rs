use std::collections::HashMap;

use gloo::storage::{errors::StorageError, LocalStorage, SessionStorage, Storage};
use tracing::warn;

use crate::{executor::CmpInOut, message::*};

use super::{Config, ConfigKind};

pub async fn fn_process<TMsg>(config: Config<TMsg>, mut in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    load_from_storage(&config, &in_out).await?;
    save_to_storage(&config, &mut in_out).await?;
    Ok(())
}

/// Сохранение в LocalStorage
async fn save_to_storage<TMsg>(
    config: &Config<TMsg>,
    in_out: &mut CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let result = (config.fn_input)(msg);
        let Some(msg) = result else { continue };
        match config.kind {
            ConfigKind::LocalStorage => LocalStorage::set(msg.key.clone(), msg)?,
            ConfigKind::SessionStorage => SessionStorage::set(msg.key.clone(), msg)?,
        };
    }
    Ok(())
}

/// Загрузка из LocalStorage
async fn load_from_storage<TMsg>(
    config: &Config<TMsg>,
    in_out: &CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let msgs: Result<HashMap<String, Message<TMsg>>, StorageError> = match config.kind {
        ConfigKind::LocalStorage => LocalStorage::get_all(),
        ConfigKind::SessionStorage => SessionStorage::get_all(),
    };
    let msgs = match msgs {
        Ok(val) => val,
        Err(err) => {
            warn!("Error loading messages from webstorage: {:?}", err);
            return Ok(());
        }
    };
    for msg in msgs.values().cloned() {
        let msg = (config.fn_output)(msg);
        let Some(msg) = msg else { continue };
        in_out.send_output(msg).await?;
    }
    Ok(())
}
