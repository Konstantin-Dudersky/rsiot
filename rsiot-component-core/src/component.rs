use tokio::{spawn, task::JoinHandle};

use crate::{
    icomponent_function::IComponentFunction,
    types::{StreamInput, StreamOutput},
    IComponent,
};

/// Обобщенный компонент
pub struct Component<TMessage, TConfig> {
    pub stream_input: StreamInput<TMessage>,
    pub stream_output: StreamOutput<TMessage>,
    pub config: TConfig,
    pub function: Box<dyn IComponentFunction<TMessage, TConfig>>,
}

impl<TMessage, TConfig> Component<TMessage, TConfig> {
    pub fn new(
        config: TConfig,
        func: impl IComponentFunction<TMessage, TConfig> + 'static,
    ) -> Self {
        Self {
            stream_input: None,
            stream_output: None,
            config,
            function: Box::new(func),
        }
    }
}

impl<TMessage, TConfig> IComponent<TMessage> for Component<TMessage, TConfig>
where
    TConfig: Clone,
{
    fn set_input(&mut self, stream_input: StreamInput<TMessage>) {
        self.stream_input = stream_input;
    }

    fn set_output(&mut self, stream_output: StreamOutput<TMessage>) {
        self.stream_output = stream_output;
    }

    fn spawn(&mut self) -> JoinHandle<()> {
        let stream_input = self.stream_input.take();
        let stream_output = self.stream_output.take();
        let config = self.config.clone();
        spawn(self.function.call(stream_input, stream_output, config))
    }
}
