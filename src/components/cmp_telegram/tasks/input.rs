use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::TelegramBot;

pub struct Input<TMsg> {
    pub input: CmpInOut<TMsg>,
    pub bot: TelegramBot,
    pub fn_input: fn(Message<TMsg>) -> Option<String>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv_input().await {
            let msg = (self.fn_input)(msg);
            let Some(msg) = msg else { continue };
            self.bot.send_message(&msg).await;
        }

        Err(super::Error::TaskEndInput)
    }
}
