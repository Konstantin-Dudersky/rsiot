use tokio::{spawn, task::JoinHandle};

use crate::{
    icomponent_function::IComponentFunction,
    types::{ComponentInput, ComponentOutput},
    CacheType, IComponent,
};

/// Обобщенный компонент
pub struct Component<TMessage, TConfig> {
    pub input: Option<ComponentInput<TMessage>>,
    pub output: Option<ComponentOutput<TMessage>>,
    pub config: Option<TConfig>,
    pub function: Box<dyn IComponentFunction<TMessage, TConfig>>,
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
            function: Box::new(func),
            cache: None,
        }
    }
}

impl<TMessage, TConfig> IComponent<TMessage> for Component<TMessage, TConfig> {
    fn set_input(&mut self, stream_input: ComponentInput<TMessage>) {
        self.input = Some(stream_input);
    }

    fn set_output(&mut self, stream_output: ComponentOutput<TMessage>) {
        self.output = Some(stream_output);
    }

    fn set_cache(&mut self, cache: crate::CacheType<TMessage>) {
        self.cache = Some(cache);
    }

    fn spawn(&mut self) -> JoinHandle<()> {
        let input = self.input.take().unwrap();
        let output = self.output.take().unwrap();
        let config = self.config.take().unwrap();
        let cache = self.cache.take().unwrap();
        spawn(self.function.call(input, output, config, cache))
    }
}
// TODO - удалить unwrap. Возможно дропать компонент в spawn()?
