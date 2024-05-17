use tokio::{fs::write, task::JoinSet};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, ConfigInput};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    for config_input in config.fn_input {
        task_set.spawn(task_input(config_input, in_out.clone()));
    }

    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    Ok(())
}

async fn task_input<TMsg>(
    config: ConfigInput<TMsg>,
    mut in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let file_content = (config.fn_save)(msg);
        let Some(file_content) = file_content else {
            continue;
        };
        write(&config.filename, file_content).await.unwrap();
    }
    Ok(())
}
