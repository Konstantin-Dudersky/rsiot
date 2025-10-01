use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_inject_single
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Создание исходящих сообщений
    pub fn_output: fn() -> Vec<TMsg>,
}
