use async_trait::async_trait;

use crate::{
    components_config::websocket_general::WebsocketMessage,
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_websocket_client";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TServerToClient, TClientToServer>
    IComponentProcess<Config<TMessage, TServerToClient, TClientToServer>, TMessage>
    for Component<Config<TMessage, TServerToClient, TClientToServer>, TMessage>
where
    TMessage: MsgDataBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    async fn process(
        &self,
        config: Config<TMessage, TServerToClient, TClientToServer>,
        input: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        let (input, output) = input.msgbus_input_output(COMPONENT_NAME);
        fn_process(input, output, config).await?;
        Ok(())
    }
}

/// Компонент cmp_websocket_client
pub type Cmp<TMessage, TServerToClient, TClientToServer> =
    Component<Config<TMessage, TServerToClient, TClientToServer>, TMessage>;
