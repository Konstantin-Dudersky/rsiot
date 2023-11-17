use tokio::task::JoinHandle;

use crate::types::{StreamInput, StreamOutput};

pub trait IComponent<TMessage> {
    /// Задать входной поток
    fn set_stream_input(&mut self, stream_input: StreamInput<TMessage>);

    /// Задать выходной поток
    fn set_stream_output(&mut self, stream_output: StreamOutput<TMessage>);

    /// Порождаем асинхронную задачу
    fn spawn(&mut self) -> JoinHandle<()>;
}

#[cfg(test)]
mod tests {
    use tokio::spawn;

    use super::*;

    struct TestComponent<TMessage> {
        stream_input: Option<StreamInput<TMessage>>,
        stream_output: Option<StreamOutput<TMessage>>,
        config: u16,
    }

    impl<TMessage> TestComponent<TMessage> {
        pub fn new(config: u16) -> Self {
            Self {
                stream_input: None,
                stream_output: None,
                config,
            }
        }
    }

    impl<TMessage> IComponent<TMessage> for TestComponent<TMessage> {
        fn spawn(&mut self) -> JoinHandle<()> {
            spawn(async move {})
        }

        fn set_stream_input(&mut self, stream_input: StreamInput<TMessage>) {
            self.stream_input = Some(stream_input);
        }

        fn set_stream_output(&mut self, stream_output: StreamOutput<TMessage>) {
            self.stream_output = Some(stream_output);
        }
    }
}
