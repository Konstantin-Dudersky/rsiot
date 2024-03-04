use async_trait::async_trait;
use serde::Serialize;

use rsiot_component_core::{Cache, CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::{AuthPermissions, MsgDataBound};

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
    TMsg: MsgDataBound + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    async fn process(
        &self,
        config: Config<TMsg, I, Q, S>,
        in_out: CmpInOut<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            in_out.clone_with_new_id("cmp_plc", AuthPermissions::FullAccess),
            config,
            cache,
        )
        .await
    }
}

pub type Cmp<TMsg, I, Q, S> = Component<Config<TMsg, I, Q, S>, TMsg>;
