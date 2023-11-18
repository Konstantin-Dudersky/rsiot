use tokio::task::JoinHandle;

use crate::types::{StreamInput, StreamOutput};

/// Трейт для работы с компонентом из построителя цепочки компонентов
/// ComponentChainBuilder
pub trait IComponent<TMessage> {
    /// Задать входной поток
    fn set_stream_input(&mut self, stream_input: StreamInput<TMessage>);

    /// Задать выходной поток
    fn set_stream_output(&mut self, stream_output: StreamOutput<TMessage>);

    /// Порождаем асинхронную задачу
    fn spawn(&mut self) -> JoinHandle<()>;
}
