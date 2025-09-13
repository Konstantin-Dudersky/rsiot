use crate::{
    executor::{Cache, CmpInOut},
    message::MsgDataBound,
};

pub struct SaveInputInCache<TMsg>
where
    TMsg: MsgDataBound,
{
    pub in_out: CmpInOut<TMsg>,
    pub input_msg_cache: Cache<TMsg>,
}

impl<TMsg> SaveInputInCache<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.in_out.recv_input().await {
            self.input_msg_cache.insert(msg).await
        }
        Err(super::Error::TaskSaveInputInCacheEnd)
    }
}
