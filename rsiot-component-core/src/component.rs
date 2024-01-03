use rsiot_messages_core::IMessage;
use tokio::{spawn, task::JoinHandle};

use crate::{
    error::Error,
    icomponent_function::IComponentFunction,
    types::{ComponentInput, ComponentOutput},
    CacheType, IComponent,
};

/// Обобщенный компонент
pub struct Component<TMessage, TConfig> {
    pub input: Option<ComponentInput<TMessage>>,
    pub output: Option<ComponentOutput<TMessage>>,
    pub config: Option<TConfig>,
    pub function: Option<Box<dyn IComponentFunction<TMessage, TConfig>>>,
    cache: Option<CacheType<TMessage>>,
}

impl<TMessage, TConfig> Component<TMessage, TConfig> {
    pub fn new(
        config: TConfig,
        func: impl IComponentFunction<TMessage, TConfig> + 'static,
    ) -> Self {
        Self {
            input: None,
            output: None,
            config: Some(config),
            function: Some(Box::new(func)),
            cache: None,
        }
    }
}

impl<TMessage, TConfig> IComponent<TMessage> for Component<TMessage, TConfig>
where
    TMessage: IMessage,
{
    fn set_input(&mut self, stream_input: ComponentInput<TMessage>) {
        self.input = Some(stream_input);
    }

    fn set_output(&mut self, stream_output: ComponentOutput<TMessage>) {
        self.output = Some(stream_output);
    }

    fn set_cache(&mut self, cache: crate::CacheType<TMessage>) {
        self.cache = Some(cache);
    }

    fn spawn(&mut self) -> Result<JoinHandle<()>, Error> {
        let input = self.input.take().ok_or(Error::InputNotSet)?;
        let output = self.output.take().ok_or(Error::OutputNotSet)?;
        let config = self.config.take().ok_or(Error::ConfigNotSet)?;
        let cache = self.cache.take().ok_or(Error::CacheNotSet)?;
        let func = self.function.take().ok_or(Error::FunctionNotSet)?;
        let handle = spawn(func.call(input, output, config, cache));
        Ok(handle)
    }
}
