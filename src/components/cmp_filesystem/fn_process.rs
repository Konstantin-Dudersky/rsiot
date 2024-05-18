use tokio::{
    fs::{read, write},
    task::JoinSet,
};
use tracing::warn;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, ConfigLoad, ConfigSave};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    for config_save in config.fn_input {
        task_set.spawn(task_save(config_save, in_out.clone()));
    }

    for config_load in config.fn_output {
        task_set.spawn(task_load(config_load, in_out.clone()));
    }

    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    Ok(())
}

async fn task_save<TMsg>(
    config_save: ConfigSave<TMsg>,
    mut in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let file_content = (config_save.fn_save)(msg);
        let Some(file_content) = file_content else {
            continue;
        };
        write(&config_save.filename, file_content).await.unwrap();
    }
    Ok(())
}

async fn task_load<TMsg>(config_load: ConfigLoad<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let content = read(&config_load.filename).await;
    let content = match content {
        Ok(ok) => ok,
        Err(err) => {
            let err = err.to_string();
            warn!(
                "File not load. File: {}. Error: {}",
                config_load.filename, err
            );
            return Ok(());
        }
    };
    let content = String::from_utf8_lossy(&content);
    let msg = (config_load.fn_restore)(&content.to_string());
    let Some(msg) = msg else { return Ok(()) };
    in_out.send_output(msg).await.unwrap();
    Ok(())
}
