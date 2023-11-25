use rsiot_messages_core::IMessage;
use tokio::{
    spawn,
    sync::mpsc,
    task::{JoinHandle, JoinSet},
};
use tracing::debug;

use crate::{component_mpsc_to_many_mpsc, IComponent};

/// Идентификатор компонента - индекс в векторе компонентов
type Id = usize;
/// Канал связи между компонентами
type Link = (Id, Id);
/// Параметры канала mpsc
type Mpsc = (Vec<Id>, Id);
/// Параметры канала broadcast
type Broadcast = (Vec<Id>, Vec<Id>);
/// Коллекция компонентов
type CmpCollection<TMessage> = Vec<Box<dyn IComponent<TMessage>>>;

/// Объединение компонентов в одну цепочку
///
/// TODO - добавить проверку конфигурации при вызовах split(), branch() ...
///
/// # Пример
/// ```
#[doc = include_str!("../examples/example1.rs")]
/// ```

pub struct ComponentChain<TMessage>
where
    TMessage: IMessage,
{
    /// Размер буфера сообщений в канале
    buffer: usize,
    /// Вектор всех компонентов
    components: Vec<Box<dyn IComponent<TMessage>>>,
    /// Предыдущий узел
    prev_node: PrevNodeType,
    /// Вектор связей всех компонентов
    links: Vec<Link>,
    /// Идентификатор узла, после которого пошло ветвление
    split_node: Option<Id>,
    /// Вектор всех незакрытых веток
    open_branshes: Vec<Id>,
}

impl<TMessage> ComponentChain<TMessage>
where
    TMessage: IMessage + 'static,
{
    /// Создание цепочки
    pub fn new(buffer: usize) -> Self {
        Self {
            buffer,
            components: vec![],
            prev_node: PrevNodeType::default(),
            links: vec![],
            split_node: None,
            open_branshes: vec![],
        }
    }

    /// Добавить компонент
    pub fn add_cmp(mut self, component: Box<dyn IComponent<TMessage>>) -> Self {
        let id = self.components.len();

        self.components.push(component);

        match self.prev_node {
            PrevNodeType::NoNode => {
                self.prev_node = PrevNodeType::PrevNode(id);
            }
            PrevNodeType::PrevNode(prev_id) => {
                self.links.push((prev_id, id));
                self.prev_node = PrevNodeType::PrevNode(id);
            }
            PrevNodeType::OpenBranches(ids) => {
                for _id in ids {
                    self.links.push((_id, id));
                }
                self.prev_node = PrevNodeType::PrevNode(id);
            }
        }
        self
    }

    /// Разделить на параллельные пути
    pub fn split(mut self) -> Self {
        self.split_node = Some(self.components.len() - 1);
        self
    }

    /// Закончился параллельный путь
    pub fn branch(mut self) -> Self {
        self.prev_node = PrevNodeType::PrevNode(self.split_node.unwrap());
        self.open_branshes.push(self.components.len() - 1);
        self
    }

    /// Объединить параллельные пути
    pub fn join(mut self) -> Self {
        self.open_branshes.push(self.components.len() - 1);
        self.prev_node = PrevNodeType::OpenBranches(self.open_branshes.clone());
        self
    }

    /// Запустить на выполнение все компоненты. Поток ожидает выполения всех задач
    pub async fn spawn(&mut self) {
        // Преобразовываем вектор связей в вектор LinkGroup
        let link_groups = create_link_groups_based_on_links(&self.links);
        debug!("{:?}", link_groups);

        // Создаем каналы между компонентами
        let mut additional_tasks: Vec<JoinHandle<()>> = vec![];
        for lg in link_groups {
            match lg.get_channel() {
                LinkGroupToChannel::Mpsc(params) => {
                    create_channels_mpsc(self.buffer, params, &mut self.components);
                }
                LinkGroupToChannel::Broadcast(params) => create_channels_broadcast(
                    self.buffer,
                    params,
                    &mut self.components,
                    &mut additional_tasks,
                ),
            }
        }

        let mut set = JoinSet::new();
        while let Some(mut cmp) = self.components.pop() {
            set.spawn(cmp.spawn());
        }
        while let Some(add) = additional_tasks.pop() {
            set.spawn(add);
        }
        while (set.join_next().await).is_some() {}
    }
}

/// Создать структуры LinkGroup на основе простого массива связей
fn create_link_groups_based_on_links(links: &Vec<Link>) -> Vec<LinkGroup> {
    let mut link_groups: Vec<LinkGroup> = vec![];
    for link in links {
        let mut found = false;
        for l_g in link_groups.iter_mut() {
            found = l_g.try_add_link(link);
            if found {
                break;
            }
        }
        if !found {
            link_groups.push(LinkGroup::new(*link));
        }
    }
    link_groups
}

/// Создаем между компонентами каналы mpsc
fn create_channels_mpsc<TMessage>(
    buffer: usize,
    (tx_ids, rx_id): Mpsc,
    components: &mut CmpCollection<TMessage>,
) {
    let (tx, rx) = mpsc::channel::<TMessage>(buffer);
    for tx_id in tx_ids {
        components[tx_id].set_output(Some(tx.clone()));
    }
    components[rx_id].set_input(Some(rx));
}

/// Создаем между компонентами каналы, похоже на broadcast
///
/// Поскольку все компоненты для унификации работают с каналами mpsc, имитируем broadcast
/// с помощью нескольких каналов mpsc.
///
/// Например. в ситуации когда один компонент посылает данные двум ( 0 -> 1, 2), добавляется
/// промежуточная задача, которая ретранслирует данные (0 -> t, t -> 1, t -> 2)
fn create_channels_broadcast<TMessage>(
    buffer: usize,
    (tx_ids, rx_ids): Broadcast,
    components: &mut CmpCollection<TMessage>,
    additional_tasks: &mut Vec<JoinHandle<()>>,
) where
    TMessage: IMessage + 'static,
{
    let mut txs = vec![];
    for rx_id in rx_ids {
        let (tx, rx) = mpsc::channel::<TMessage>(buffer);
        components[rx_id].set_input(Some(rx));
        txs.push(tx)
    }
    let (tx, rx) = mpsc::channel::<TMessage>(buffer);
    for tx_id in tx_ids {
        components[tx_id].set_output(Some(tx.clone()));
    }
    let _th = spawn(component_mpsc_to_many_mpsc(rx, txs));
    additional_tasks.push(_th);
}

#[derive(Default)]
enum PrevNodeType {
    #[default]
    NoNode,
    PrevNode(usize),
    OpenBranches(Vec<usize>),
}

/// Группировка связей, у которых совпадает начало или конец.
///
/// Например:
/// - (2, 3), (2, 4)
/// - (4, 5), (2, 5)
#[derive(Debug)]
struct LinkGroup {
    begin: Vec<Id>,
    end: Vec<Id>,
}

impl LinkGroup {
    fn new(link: Link) -> Self {
        Self {
            begin: vec![link.0],
            end: vec![link.1],
        }
    }

    /// Пробуем добавить связь. Если id начала или конца совпадают - добавляем и возвращаем true
    fn try_add_link(&mut self, link: &Link) -> bool {
        if !self.begin.contains(&link.0) && !self.end.contains(&link.1) {
            return false;
        }
        self.begin.push(link.0);
        self.begin.dedup();
        self.end.push(link.1);
        self.end.dedup();
        true
    }

    /// Определяем, какой канал tokio подходит для данного LinkGroup
    fn get_channel(&self) -> LinkGroupToChannel {
        if self.end.len() == 1 {
            let params = (self.begin.clone(), self.end[0]);
            return LinkGroupToChannel::Mpsc(params);
        }
        let params = (self.begin.clone(), self.end.clone());
        LinkGroupToChannel::Broadcast(params)
    }
}

/// Преобразование структуры LinkGroup в каналы синхронизации tokio
enum LinkGroupToChannel {
    Mpsc(Mpsc),
    Broadcast(Broadcast),
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::component_example;

    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    enum TestMessage {}

    impl IMessage for TestMessage {}

    #[tokio::test]
    async fn test1() {
        let _chain = ComponentChain::<TestMessage>::new(100)
            .add_cmp(component_example::new())
            .add_cmp(component_example::new())
            .split()
            .add_cmp(component_example::new())
            .add_cmp(component_example::new())
            .branch()
            .add_cmp(component_example::new())
            .add_cmp(component_example::new())
            .branch()
            .add_cmp(component_example::new())
            .add_cmp(component_example::new())
            .add_cmp(component_example::new())
            .join()
            .add_cmp(component_example::new())
            .add_cmp(component_example::new());
    }
}
