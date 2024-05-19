use tokio::{fs::create_dir_all, task::JoinSet};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{tasks, Config, Error};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    create_dir_all(&config.directory)
        .await
        .map_err(Error::CreateDirError)?;

    // Загрузка сообщений из файловой системы - выполняем один раз
    tasks::output(config.directory.clone(), config.fn_output, in_out.clone()).await?;

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    task_set.spawn(tasks::input(config.directory, config.fn_input, in_out));

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
