use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use crate::{executor::MsgBusInput, message::MsgDataBound};

use super::super::config::FnFindPartnerCounter;

pub struct FindPartnerCounter<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub fn_find_partner_counter: FnFindPartnerCounter<TMsg>,
    pub live_counter: Arc<AtomicU8>,
}

impl<TMsg> FindPartnerCounter<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv().await {
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
