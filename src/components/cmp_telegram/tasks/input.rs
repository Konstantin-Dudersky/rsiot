use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound, ServiceBound},
};

use super::TelegramBot;

pub struct Input<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub input: CmpInOut<TMsg, TService>,
    pub bot: TelegramBot,
    pub fn_input: fn(Message<TMsg>) -> Option<String>,
}

impl<TMsg, TService> Input<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
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
