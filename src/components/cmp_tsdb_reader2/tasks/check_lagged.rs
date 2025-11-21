use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{
    executor::MsgBusInput,
    message::{MsgData, MsgDataBound, system_messages::System},
};

use super::Error;

pub struct CheckLagged<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub input_lagged: Arc<AtomicBool>,
}

impl<TMsg> CheckLagged<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.input.recv().await {
            if let MsgData::System(System::InputChannelFull) = msg.data {
                self.input_lagged.store(true, Ordering::Release);
            }
        }

        Ok(())
    }
}
