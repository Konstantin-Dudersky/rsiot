use tokio::{
    spawn,
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use rsiot_messages_core::IMessage;

use crate::{
    cmpbase_cache::{self, cmpbase_cache},
    cmpbase_mpsc_to_broadcast, IComponent,
};

/// Объединение компонентов в одну цепочку
///
/// # Пример
/// ```
#[doc = include_str!("../examples/example1.rs")]
/// ```
pub struct ComponentCollection<TMessage>
where
    TMessage: IMessage,
{
    /// Размер буфера сообщений в канале
    buffer_size: usize,
    /// Вектор всех компонентов
    components: Vec<Box<dyn IComponent<TMessage>>>,
}

impl<TMessage> ComponentCollection<TMessage>
where
    TMessage: IMessage + 'static,
{
    /// Создание цепочки
    pub fn new(buffer_size: usize, components: Vec<Box<dyn IComponent<TMessage>>>) -> Self {
        Self {
            buffer_size,
            components,
        }
    }

    /// Запустить на выполнение все компоненты. Поток ожидает выполения всех задач
    pub async fn spawn(&mut self) {
        let (input_tx, _input_rx) = broadcast::channel(self.buffer_size);
        let (output_tx, output_rx) = mpsc::channel(self.buffer_size);

        let cache = cmpbase_cache::create_cache();
        let task_cache = cmpbase_cache(
            input_tx.subscribe(),
            cmpbase_cache::Config {
                cache: cache.clone(),
            },
        );
        spawn(task_cache);

        for component in self.components.iter_mut() {
            component.set_input(input_tx.subscribe());
            component.set_output(output_tx.clone());
            component.set_cache(cache.clone());
        }

        let mut set = JoinSet::new();
        while let Some(mut cmp) = self.components.pop() {
            set.spawn(cmp.spawn());
        }
        let task = cmpbase_mpsc_to_broadcast::new(output_rx, input_tx);
        spawn(task);

        while (set.join_next().await).is_some() {}
    }
}