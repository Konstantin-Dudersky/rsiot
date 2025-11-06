use async_trait::async_trait;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
    message::*,
};

use super::{BufferBound, Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_derive";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TBuffer> IComponentProcess<Config<TMsg, TBuffer>, TMsg>
    for Component<Config<TMsg, TBuffer>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TBuffer: 'static + BufferBound,
{
    async fn process(
        &self,
        config: Config<TMsg, TBuffer>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(msgbus_linker.init(COMPONENT_NAME), config)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

/// Компонент cmp_derive
pub type Cmp<TMsg, TBuffer> = Component<Config<TMsg, TBuffer>, TMsg>;
