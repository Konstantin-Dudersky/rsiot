use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{fn_process::fn_process, Config, StoreBound};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TInputStore, TOutputStore>
    IComponentProcess<Config<TMsg, TInputStore, TOutputStore>, TMsg>
    for Component<Config<TMsg, TInputStore, TOutputStore>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TInputStore: StoreBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TInputStore, TOutputStore>,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            input.clone_with_new_id("cmp_leptos", AuthPermissions::FullAccess),
        )
        .await?;
        Err(ComponentError::Execution("Stop execution".into()))
    }
}

/// Компонент cmp_leptos
pub type Cmp<TMsg, TInputStore, TOutputStore> =
    Component<Config<TMsg, TInputStore, TOutputStore>, TMsg>;
