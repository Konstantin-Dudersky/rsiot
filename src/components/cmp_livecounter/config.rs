use std::time::Duration;

use crate::message::{Message, MsgDataBound};

pub type FnFindPartnerCounter<TMsg> = fn(&Message<TMsg>) -> Option<u8>;
pub type FnCheckPartnerCounter<TMsg> = fn(bool) -> Message<TMsg>;
pub type FnGenerateSelfCounter<TMsg> = fn(u8) -> Message<TMsg>;

/// Конфигурация cmp_check_livecounter
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Функция для формирования сообщения о собственном счетчике
    pub fn_generate_self_counter: FnGenerateSelfCounter<TMsg>,

    /// Период создания сообщения о собственном счетчике
    pub generate_self_period: Duration,

    /// Ищем сообщения со значением счетчика
    ///
    /// Заглушка: `|_| None`
    pub fn_find_partner_counter: FnFindPartnerCounter<TMsg>,

    /// Функция для формирования сообщения о наличии связи. В функцию передается булевое значение,
    /// указывающее на наличие связи. true - есть связь, false - нет связи.
    pub fn_check_partner_counter: FnCheckPartnerCounter<TMsg>,

    /// Период проверки счетчика на наличие связи
    pub check_partner_period: Duration,
}
