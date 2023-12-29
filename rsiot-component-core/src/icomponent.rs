use tokio::task::JoinHandle;

use crate::{
    types::{ComponentInput, ComponentOutput},
    CacheType,
};

/// Трейт для работы с компонентом из построителя цепочки компонентов
/// ComponentChainBuilder
pub trait IComponent<TMessage> {
    /// Задать входной поток
    fn set_input(&mut self, input: ComponentInput<TMessage>);

    /// Задать выходной поток
    fn set_output(&mut self, output: ComponentOutput<TMessage>);

    // Задать ссылку на кеш
    fn set_cache(&mut self, cache: CacheType<TMessage>);

    /// Порождаем асинхронную задачу
    fn spawn(&mut self) -> JoinHandle<()>;

    /// Задать входной и выходной потоки и запустить на выполнение
    fn set_and_spawn(
        &mut self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
    ) -> JoinHandle<()> {
        self.set_input(input);
        self.set_output(output);
        self.spawn()
    }
}
