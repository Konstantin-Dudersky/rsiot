use async_trait::async_trait;
use serde::Serialize;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{
    config::Config,
    fn_process::fn_process,
    plc::{FunctionBlockBase, IFunctionBlock},
};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_plc";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(? Send))]
impl<TMsg, I, Q, S> IComponentProcess<Config<TMsg, I, Q, S>, TMsg>
    for Component<Config<TMsg, I, Q, S>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    async fn process(
        &self,
        config: Config<TMsg, I, Q, S>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(msgbus_linker.init(COMPONENT_NAME), config)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

/// Компонент cmp_plc
pub type Cmp<TMsg, I, Q, S> = Component<Config<TMsg, I, Q, S>, TMsg>;
