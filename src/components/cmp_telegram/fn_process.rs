use std::sync::Arc;

use teloxide::prelude::*;
use teloxide::prelude::{Bot, ChatId};
use tokio::sync::Mutex;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(config: Config<TMsg>, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let bot = Arc::new(Mutex::new(Bot::new(config.bot_token)));

    task_fn_input(bot).await?;
    Ok(())
}

async fn task_fn_input(bot: Arc<Mutex<Bot>>) -> super::Result<()> {
    let bot = bot.lock().await;
    bot.send_message(ChatId(-1002220119164), " ⚠️ ❗ alarm!!")
        .await
        .unwrap();
    Ok(())
}
