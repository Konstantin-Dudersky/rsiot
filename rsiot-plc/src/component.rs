use async_trait::async_trait;
use serde::Serialize;

use rsiot_component_core::{
    Cache, Component, ComponentError, ComponentInput, ComponentOutput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{
    config::Config,
    fn_process::fn_process,
    plc::function_block_base::{FunctionBlockBase, IFunctionBlock},
};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(? Send))]
impl<TMsg, I, Q, S> IComponentProcess<Config<TMsg, I, Q, S>, TMsg>
    for Component<Config<TMsg, I, Q, S>, TMsg>
where
    TMsg: IMessage + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    async fn process(
        &self,
        config: Config<TMsg, I, Q, S>,
        _input: ComponentInput<TMsg>,
        output: ComponentOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(output, config, cache).await
    }
}

pub type Cmp<TMsg, I, Q, S> = Component<Config<TMsg, I, Q, S>, TMsg>;
