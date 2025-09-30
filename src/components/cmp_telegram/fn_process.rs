use tokio::task::JoinSet;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, TelegramBot, tasks};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let bot = TelegramBot::new(config.bot_token, config.chat_id);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Обработка входящих сообщений
    let task = tasks::Input {
        input: msgbus_linker.input(),
        bot,
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_telegram", task.spawn());

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}
