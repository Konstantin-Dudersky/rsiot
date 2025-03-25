use tokio::task::JoinSet;

use crate::{
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{tasks, Config, TelegramBot};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let bot = TelegramBot::new(config.bot_token, config.chat_id);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Обработка входящих сообщений
    let task = tasks::Input {
        input: msg_bus,
        bot,
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    Ok(())
}
