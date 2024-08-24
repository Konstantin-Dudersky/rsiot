use std::collections::HashMap;

use gloo::storage::{LocalStorage, SessionStorage, Storage};

use crate::{executor::CmpInOut, message::*};

use super::{Config, ConfigKind, ConfigWebstorageItem};

pub async fn fn_process<TMsg>(config: Config<TMsg>, mut in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    load_from_storage(&config, &in_out).await?;
    while let Ok(msg) = in_out.recv_input().await {
        let item = (config.fn_input)(msg).map_err(super::Error::FnInput)?;
        let Some(item) = item else { continue };
        match config.kind {
            ConfigKind::LocalStorage => LocalStorage::set(item.key, item.value)?,
            ConfigKind::SessionStorage => SessionStorage::set(item.key, item.value)?,
        };
        load_from_storage(&config, &in_out).await?;
    }
    Ok(())
}

/// Загрузка из WebStorage
async fn load_from_storage<TMsg>(
    config: &Config<TMsg>,
    in_out: &CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut items: HashMap<String, String> = match config.kind {
        ConfigKind::LocalStorage => LocalStorage::get_all()?,
        ConfigKind::SessionStorage => SessionStorage::get_all()?,
    };

    // Добавляем значения по-умолчанию
    for ConfigWebstorageItem { key, value } in &config.default_items {
        if !items.contains_key(key) {
            items.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in items.into_iter() {
        let item = ConfigWebstorageItem { key, value };
        let msg = (config.fn_output)(item).map_err(super::Error::FnOutput)?;
        let Some(msg) = msg else { continue };
        in_out.send_output(msg).await?;
    }
    Ok(())
}
