use async_trait::async_trait;

use rsiot_messages_core::IMessage;

use crate::{Cache, ComponentError, ComponentInput, ComponentOutput};

pub struct Component<TConfig, TMessage>
where
    TMessage: IMessage,
{
    input: Option<ComponentInput<TMessage>>,
    output: Option<ComponentOutput<TMessage>>,
    cache: Option<Cache<TMessage>>,
    config: Option<TConfig>,
}

impl<TConfig, TMessage> Component<TConfig, TMessage>
where
    TMessage: IMessage,
{
    pub fn new(config: TConfig) -> Self {
        Self {
            input: None,
            output: None,
            cache: None,
            config: Some(config),
        }
    }
}

#[async_trait]
impl<TConfig, TMessage> IComponent<TMessage> for Component<TConfig, TMessage>
where
    TMessage: IMessage,
    Self: IComponentProcess<TConfig, TMessage>,
    TConfig: Send,
{
    fn set_interface(
        &mut self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        cache: Cache<TMessage>,
    ) {
        self.input = Some(input);
        self.output = Some(output);
        self.cache = Some(cache);
    }

    async fn spawn(&mut self) -> Result<(), ComponentError> {
        let input = self
            .input
            .take()
            .ok_or(ComponentError::Initialization("input not set".into()))?;

        let output = self
            .output
            .take()
            .ok_or(ComponentError::Initialization("output not set".into()))?;

        let config = self
            .config
            .take()
            .ok_or(ComponentError::Initialization("config not set".into()))?;

        let cache = self
            .cache
            .take()
            .ok_or(ComponentError::Initialization("cache not set".into()))?;

        self.process(config, input, output, cache).await
    }
}

#[async_trait]
pub trait IComponentProcess<TConfig, TMessage>
where
    TMessage: IMessage,
{
    async fn process(
        &self,
        config: TConfig,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError>;
}

#[async_trait]
pub trait IComponent<TMessage>
where
    TMessage: IMessage,
{
    fn set_interface(
        &mut self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        cache: Cache<TMessage>,
    );

    async fn spawn(&mut self) -> Result<(), ComponentError>;
}
