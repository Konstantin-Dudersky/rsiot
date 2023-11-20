use tokio::task::JoinHandle;

use crate::types::{StreamInput, StreamOutput};

/// Трейт для работы с компонентом из построителя цепочки компонентов
/// ComponentChainBuilder
pub trait IComponent<TMessage> {
    /// Задать входной поток
    fn set_input(&mut self, input: StreamInput<TMessage>);

    /// Задать выходной поток
    fn set_output(&mut self, output: StreamOutput<TMessage>);

    /// Порождаем асинхронную задачу
    fn spawn(&mut self) -> JoinHandle<()>;

    /// Задать входной и выходной потоки и запустить на выполнение
    fn set_and_spawn(
        &mut self,
        input: StreamInput<TMessage>,
        output: StreamOutput<TMessage>,
    ) -> JoinHandle<()> {
        self.set_input(input);
        self.set_output(output);
        self.spawn()
    }
}
