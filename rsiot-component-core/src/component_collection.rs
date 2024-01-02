use tokio::{
    spawn,
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use rsiot_messages_core::IMessage;

use crate::{cache::create_cache, types::CacheType, IComponent};

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

        let cache = create_cache();

        spawn(task_cache(output_rx, input_tx.clone(), cache.clone()));

        // let task_cache = cmpbase_cache(
        //     input_tx.subscribe(),
        //     cmpbase_cache::Config {
        //         cache: cache.clone(),
        //     },
        // );
        // spawn(task_cache);

        for component in self.components.iter_mut() {
            component.set_input(input_tx.subscribe());
            component.set_output(output_tx.clone());
            component.set_cache(cache.clone());
        }

        let mut set = JoinSet::new();
        while let Some(mut cmp) = self.components.pop() {
            set.spawn(cmp.spawn());
        }
        // let task = cmpbase_mpsc_to_broadcast::new(output_rx, input_tx);
        // spawn(task);

        while (set.join_next().await).is_some() {}
    }
}

async fn task_cache<TMessage>(
    mut input: mpsc::Receiver<TMessage>,
    output: broadcast::Sender<TMessage>,
    cache: CacheType<TMessage>,
) where
    TMessage: IMessage,
{
    while let Some(msg) = input.recv().await {
        let key = msg.key().clone();
        let value = msg.clone();
        {
            let mut lock = cache.write().await;
            let value_from_cache = lock.get(&key);
            if let Some(value_from_cache) = value_from_cache {
                // если значение эквивалентно сохраненному в кеше, переходим к ожиданию следующего
                // сообщения
                if value == *value_from_cache {
                    continue;
                }
            }
            lock.insert(key, value);
        }
        output.send(msg).unwrap();
    }
}
