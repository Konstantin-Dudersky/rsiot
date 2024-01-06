use futures::Future;
use tokio::task::JoinHandle;

use crate::{
    cache::Cache,
    error::ComponentError,
    types::{ComponentInput, ComponentOutput},
};

/// Трейт для работы с компонентом из построителя цепочки компонентов
/// ComponentChainBuilder
pub trait IComponent<TMessage> {
    /// Задать входной поток
    fn set_input(&mut self, input: ComponentInput<TMessage>);

    /// Задать выходной поток
    fn set_output(&mut self, output: ComponentOutput<TMessage>);

    // Задать ссылку на кеш
    fn set_cache(&mut self, cache: Cache<TMessage>);

    /// Порождаем асинхронную задачу
    fn spawn(&mut self) -> std::pin::Pin<Box<dyn Future<Output = Result<(), ComponentError>>>>;

    //// Задать входной и выходной потоки и запустить на выполнение
    // fn set_and_spawn(
    //     &mut self,
    //     input: ComponentInput<TMessage>,
    //     output: ComponentOutput<TMessage>,
    // ) -> Result<JoinHandle<Result<(), ComponentError>>, ComponentError> {
    //     self.set_input(input);
    //     self.set_output(output);
    //     self.spawn()
    // }
}
