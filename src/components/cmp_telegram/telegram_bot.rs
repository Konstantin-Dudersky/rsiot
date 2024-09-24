use std::sync::Arc;

use teloxide::{prelude::Requester, types::ChatId, Bot};
use tokio::sync::Mutex;
use tracing::warn;

#[derive(Clone)]
pub struct TelegramBot {
    bot: Arc<Mutex<Bot>>,
    chat_id: ChatId,
}

impl TelegramBot {
    pub fn new(bot_token: String, chat_id: i64) -> Self {
        let bot = Arc::new(Mutex::new(Bot::new(bot_token)));
        let chat_id = ChatId(chat_id);
        Self { bot, chat_id }
    }

    pub async fn send_message(&self, message: &str) {
        let bot = self.bot.lock().await;
        let res = bot.send_message(self.chat_id, message).await;
        if let Err(err) = res {
            warn!("cmp_telegram error: {}", err);
        }
    }
}
// " ⚠️ ❗ alarm!!"
