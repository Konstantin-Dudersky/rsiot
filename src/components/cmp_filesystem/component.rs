use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process, BufferBound};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TBuffer> IComponentProcess<Config<TMsg, TBuffer>, TMsg>
    for Component<Config<TMsg, TBuffer>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TBuffer: BufferBound,
{
    async fn process(&self, config: Config<TMsg, TBuffer>, in_out: CmpInOut<TMsg>) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_filesystem", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_filesystem
pub type Cmp<TMsg, TBuffer> = Component<Config<TMsg, TBuffer>, TMsg>;
