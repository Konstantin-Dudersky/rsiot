use async_trait::async_trait;

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{BufferBound, config::Config, fn_process::fn_process};

pub const COMPONENT_NAME: &str = "cmp_filesystem";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TBuffer> IComponentProcess<Config<TMsg, TBuffer>, TMsg>
    for Component<Config<TMsg, TBuffer>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TBuffer: BufferBound,
{
    async fn process(
        &self,
        config: Config<TMsg, TBuffer>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_filesystem
pub type Cmp<TMsg, TBuffer> = Component<Config<TMsg, TBuffer>, TMsg>;
