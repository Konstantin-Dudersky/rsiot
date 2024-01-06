use futures::{future::LocalBoxFuture, Future};
use rsiot_messages_core::IMessage;

use crate::{
    error::ComponentError,
    // icomponent_function::IComponentFunction,
    types::{ComponentInput, ComponentOutput},
    Cache,
    IComponent,
};

pub type FuncType = fn() -> LocalBoxFuture<'static, Result<(), ComponentError>>;

/// Обобщенный компонент
pub struct Component<TMessage, TConfig> {
    pub input: Option<ComponentInput<TMessage>>,
    pub output: Option<ComponentOutput<TMessage>>,
    pub config: Option<TConfig>,
    // pub function: Option<Box<dyn Future<Output = Result<(), ComponentError>>>>,
    pub function: Option<FuncType>,
    cache: Option<Cache<TMessage>>,
}

impl<TMessage, TConfig> Component<TMessage, TConfig> {
    pub fn new(
        config: TConfig,
        // func: impl IComponentFunction<TMessage, TConfig> + 'static,
        func: FuncType,
    ) -> Self {
        Self {
            input: None,
            output: None,
            config: Some(config),
            function: Some(func),
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

    fn set_cache(&mut self, cache: crate::Cache<TMessage>) {
        self.cache = Some(cache);
    }

    fn spawn(&mut self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ComponentError>>>> {
        let input = self
            .input
            .take()
            .ok_or(ComponentError::Initialization("input not set".into()))
            .unwrap();

        let output = self
            .output
            .take()
            .ok_or(ComponentError::Initialization("output not set".into()))
            .unwrap();

        let config = self
            .config
            .take()
            .ok_or(ComponentError::Initialization("config not set".into()))
            .unwrap();

        let cache = self
            .cache
            .take()
            .ok_or(ComponentError::Initialization("cache not set".into()))
            .unwrap();

        let func = self
            .function
            .take()
            .ok_or(ComponentError::Initialization("function not set".into()))
            .unwrap();

        // func(input, output, config, cache);
        (func)()
    }
}
