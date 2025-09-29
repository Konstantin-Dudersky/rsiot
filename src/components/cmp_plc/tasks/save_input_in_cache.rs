use crate::{
    executor::{Cache, MsgBusInput},
    message::MsgDataBound,
};

pub struct SaveInputInCache<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub input_msg_cache: Cache<TMsg>,
}

impl<TMsg> SaveInputInCache<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv().await {
            self.input_msg_cache.insert(msg).await
        }
        Err(super::Error::TaskSaveInputInCacheEnd)
    }
}
