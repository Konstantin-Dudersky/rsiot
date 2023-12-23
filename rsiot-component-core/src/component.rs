use tokio::{spawn, task::JoinHandle};

use crate::{
    icomponent_function::IComponentFunction,
    types::{Input, Output},
    IComponent,
};

/// Обобщенный компонент
pub struct Component<TMessage, TConfig> {
    pub input: Option<Input<TMessage>>,
    pub output: Option<Output<TMessage>>,
    pub config: Option<TConfig>,
    pub function: Box<dyn IComponentFunction<TMessage, TConfig>>,
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
        }
    }
}

impl<TMessage, TConfig> IComponent<TMessage> for Component<TMessage, TConfig> {
    fn set_input(&mut self, stream_input: Input<TMessage>) {
        self.input = Some(stream_input);
    }

    fn set_output(&mut self, stream_output: Output<TMessage>) {
        self.output = Some(stream_output);
    }

    fn spawn(&mut self) -> JoinHandle<()> {
        let input = self.input.take().unwrap();
        let output = self.output.take().unwrap();
        let config = self.config.take().unwrap();
        spawn(self.function.call(input, output, config))
    }
}
// TODO - удалить unwrap. Возможно дропать компонент в spawn()?
