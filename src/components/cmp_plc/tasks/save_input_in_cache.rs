use crate::{
    executor::{Cache, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

pub struct SaveInputInCache<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub in_out: CmpInOut<TMsg, TService>,
    pub input_msg_cache: Cache<TMsg>,
}

impl<TMsg, TService> SaveInputInCache<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.in_out.recv_input().await {
            self.input_msg_cache.insert(msg).await
        }
        Ok(())
    }
}
