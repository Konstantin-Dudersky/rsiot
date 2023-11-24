use rsiot_messages_core::IMessage;
use tokio::{
    spawn,
    sync::mpsc,
    task::{JoinHandle, JoinSet},
};

use crate::{component_mpsc_to_many_mpsc, IComponent};

/// Идентификатор компонента - индекс в векторе компонентов
type Id = usize;

/// Канал связи между компонентами
type Link = (Id, Id);

/// Параметры канала mpsc
type Mpsc = (Vec<Id>, Id);

/// Параметры канала broadcast
type Broadcast = (Vec<Id>, Vec<Id>);

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
    components: Vec<Box<dyn IComponent<TMessage>>>,
    /// -- experiment
    prev_node: PrevNodeType,
    transitions: Vec<Link>,
    split_node: Option<Id>,
    open_branshes: Vec<Id>,
    additional_tasks: Vec<JoinHandle<()>>,
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
            transitions: vec![],
            split_node: None,
            open_branshes: vec![],
            additional_tasks: vec![],
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
                self.transitions.push((prev_id, id));
                self.prev_node = PrevNodeType::PrevNode(id);
            }
            PrevNodeType::OpenBranches(ids) => {
                for _id in ids {
                    self.transitions.push((_id, id));
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

    pub fn build(mut self) -> Self {
        let mut tr_gr: Vec<LinkGroup> = vec![];
        for tr in &self.transitions {
            let mut found = false;
            for tr_g in tr_gr.iter_mut() {
                if tr_g.begin.contains(&tr.0) {
                    tr_g.add_link(*tr);
                    found = true;
                    break;
                }
                if tr_g.end.contains(&tr.1) {
                    tr_g.add_link(*tr);
                    found = true;
                    break;
                }
            }
            if !found {
                tr_gr.push(LinkGroup::new(*tr));
            }
        }
        println!("{:?}", tr_gr);
        for lg in tr_gr {
            match lg.get_channel() {
                Channel::Mpsc((tx_ids, rx_id)) => {
                    let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
                    for tx_id in tx_ids {
                        self.components[tx_id].set_output(Some(tx.clone()));
                    }
                    self.components[rx_id].set_input(Some(rx));
                }
                Channel::Broadcast((tx_ids, rx_ids)) => {
                    let mut txs = vec![];
                    for rx_id in rx_ids {
                        let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
                        self.components[rx_id].set_input(Some(rx));
                        txs.push(tx)
                    }
                    let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
                    for tx_id in tx_ids {
                        self.components[tx_id].set_output(Some(tx.clone()));
                    }
                    let _th = spawn(component_mpsc_to_many_mpsc(rx, txs));
                    self.additional_tasks.push(_th);
                }
            }
        }
        self
    }

    /// Запустить на выполнение все компоненты. Поток ожидает выполения
    /// всех задач
    /// TODO - обработка ошибок
    pub async fn spawn(&mut self) {
        let mut set = JoinSet::new();
        for cmp in self.components.iter_mut() {
            set.spawn(cmp.spawn());
        }
        while self.additional_tasks.len() > 0 {
            let a = self.additional_tasks.pop().unwrap();
            set.spawn(a);
        }
        while (set.join_next().await).is_some() {}
    }
}
#[derive(Default)]
enum PrevNodeType {
    #[default]
    NoNode,
    PrevNode(usize),
    OpenBranches(Vec<usize>),
}

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
    fn add_link(&mut self, link: Link) {
        self.begin.push(link.0);
        self.begin.dedup();
        self.end.push(link.1);
        self.end.dedup();
    }

    fn get_channel(&self) -> Channel {
        if self.end.len() == 1 {
            Channel::Mpsc((self.begin.clone(), self.end[0]));
        }
        Channel::Broadcast((self.begin.clone(), self.end.clone()))
    }
}

enum Channel {
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
            .add_cmp(component_example::new())
            .build();
    }
}
