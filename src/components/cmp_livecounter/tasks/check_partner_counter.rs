use std::{
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
    time::Duration,
};

use tokio::time::sleep;

use crate::{executor::MsgBusOutput, message::MsgDataBound};

use super::{super::config::FnCheckPartnerCounter, Error};

pub struct CheckPartnerPeriod<TMsg>
where
    TMsg: MsgDataBound,
{
    pub output: MsgBusOutput<TMsg>,
    pub fn_check_partner_counter: FnCheckPartnerCounter<TMsg>,
    pub check_partner_period: Duration,
    pub live_counter: Arc<AtomicU8>,
}

impl<TMsg> CheckPartnerPeriod<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        let mut prev_live_counter = 0;

        loop {
            sleep(self.check_partner_period).await;

            let live_counter = self.live_counter.load(Ordering::Acquire);

            let msg = (self.fn_check_partner_counter)(live_counter != prev_live_counter);
            prev_live_counter = live_counter;

            self.output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }
    }
}
