use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;
use serde::Serialize;

use crate::{
    config::Config,
    fn_process::fn_process,
    plc::function_block_base::{FunctionBlockBase, IFunctionBlock},
};

pub fn new<TMessage, I, Q, S>(
    config: Config<TMessage, I, Q, S>,
) -> Box<Component<TMessage, Config<TMessage, I, Q, S>>>
where
    TMessage: IMessage + 'static,
    I: Clone + Default + Send + Serialize + 'static + Sync,
    Q: Clone + Default + Send + Serialize + 'static + Sync,
    S: Clone + Default + Send + Serialize + 'static + Sync,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
