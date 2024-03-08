use async_trait::async_trait;
use leptos::*;

use rsiot_messages_core::{AuthPermissions, MsgDataBound};

use crate::executor::{CmpInOut, Component, ComponentError, IComponentProcess};

use super::{fn_process::fn_process, Config};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TView, TIntoView> IComponentProcess<Config<TView, TIntoView>, TMsg>
    for Component<Config<TView, TIntoView>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
{
    async fn process(
        &self,
        config: Config<TView, TIntoView>,
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

pub type Cmp<TMsg, TView, TIntoView> = Component<Config<TView, TIntoView>, TMsg>;
