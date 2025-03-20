use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{
    config::{Config, WebsocketMessage},
    fn_process::fn_process,
};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TService, TServerToClient, TClientToServer>
    IComponentProcess<Config<TMessage, TServerToClient, TClientToServer>, TMessage, TService>
    for Component<Config<TMessage, TServerToClient, TClientToServer>, TMessage, TService>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    async fn process(
        &self,
        config: Config<TMessage, TServerToClient, TClientToServer>,
        input: CmpInOut<TMessage, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            input.clone_with_new_id("cmp_websocket_server", AuthPermissions::FullAccess),
            config,
        )
        .await
    }
}

/// Компонент cmp_websocker_server
pub type Cmp<TMessage, TService, TServerToClient, TClientToServer> =
    Component<Config<TMessage, TServerToClient, TClientToServer>, TMessage, TService>;
