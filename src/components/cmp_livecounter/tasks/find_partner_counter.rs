use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};

use tokio::sync::mpsc;

use crate::message::Message;

use super::super::config::FnFindPartnerCounter;

pub struct FindPartnerCounter<TMsg> {
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub fn_find_partner_counter: FnFindPartnerCounter<TMsg>,
    pub live_counter: Arc<AtomicU8>,
}

impl<TMsg> FindPartnerCounter<TMsg> {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.recv().await {
            let live_counter = (self.fn_find_partner_counter)(&msg);
            let live_counter = match live_counter {
                Some(val) => val,
                None => continue,
            };
            self.live_counter.store(live_counter, Ordering::Release);
        }

        Err(super::Error::TaskInput)
    }
}
