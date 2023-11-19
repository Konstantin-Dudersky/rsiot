use tokio::task::JoinHandle;

use crate::types::{StreamInput, StreamOutput};

/// Трейт для работы с компонентом из построителя цепочки компонентов
/// ComponentChainBuilder
pub trait IComponent<TMessage> {
    /// Задать входной поток
    fn set_input(&mut self, input: StreamInput<TMessage>);

    /// Задать выходной поток
    fn set_output(&mut self, output: StreamOutput<TMessage>);

    /// Задать входной и выходной потоки
    fn set_input_output(
        &mut self,
        input: StreamInput<TMessage>,
        output: StreamOutput<TMessage>,
    ) {
        self.set_input(input);
        self.set_output(output);
    }

    /// Порождаем асинхронную задачу
    fn spawn(&mut self) -> JoinHandle<()>;
}
