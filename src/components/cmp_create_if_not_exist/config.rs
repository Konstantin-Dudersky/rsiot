use std::time::Duration;

use crate::message::{Message, MsgDataBound};

/// Настройка компонента cmp_create_if_not_exist
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Задержка создания сообщений
    pub delay: Duration,

    /// Вектор сообщений, которые генерируются, если отсутствуют в кеше
    pub msgs: Vec<Message<TMsg>>,
}
