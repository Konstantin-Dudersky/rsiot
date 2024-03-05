use std::collections::HashMap;

use gloo::storage::{LocalStorage, Storage};

use rsiot_component_core::CmpInOut;
use rsiot_messages_core::*;

use crate::{Config, Error};

pub async fn fn_process<TMsg>(config: Config<TMsg>, mut in_out: CmpInOut<TMsg>) -> crate::Result<()>
where
    TMsg: MsgDataBound,
{
    load_from_storage(&config, &in_out).await?;
    save_to_storage(&config, &mut in_out).await?;
    Ok(())
}

/// Загрузка из LocalStorage
async fn load_from_storage<TMsg>(
    config: &Config<TMsg>,
    in_out: &CmpInOut<TMsg>,
) -> crate::Result<()>
where
    TMsg: MsgDataBound,
{
    let msgs: HashMap<String, String> = LocalStorage::get_all()?;
    for msg in msgs.values() {
        let msg = (config.fn_output)(msg).map_err(Error::FnOutput)?;
        let Some(msg) = msg else { continue };
        in_out.send_output(msg).await?;
    }
    Ok(())
}

/// Сохранение в LocalStorage
async fn save_to_storage<TMsg>(
    config: &Config<TMsg>,
    in_out: &mut CmpInOut<TMsg>,
) -> crate::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let text = (config.fn_input)(&msg).map_err(Error::FnInput)?;
        let Some(text) = text else { continue };
        LocalStorage::set(text.0, text.1)?;
    }
    Ok(())
}
