use std::time::Duration;

use tokio::{sync::mpsc, time::sleep};

use crate::message::Message;

use super::super::config::FnGenerateSelfCounter;

pub struct GenerateSelfCounter<TMsg> {
    pub output: mpsc::Sender<Message<TMsg>>,
    pub fn_generate_self_counter: FnGenerateSelfCounter<TMsg>,
    pub generate_self_period: Duration,
}

impl<TMsg> GenerateSelfCounter<TMsg> {
    pub async fn spawn(self) -> super::Result<()> {
        let mut self_counter: u8 = 0;

        loop {
            self_counter = self_counter.wrapping_add(1);
            let msg = (self.fn_generate_self_counter)(self_counter);
            self.output.send(msg).await.unwrap();
            sleep(self.generate_self_period).await
        }
    }
}
