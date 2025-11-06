use crate::message::{MsgDataBound, ValueTime};

use super::Algs;

// ANCHOR: Config
/// Конфигурация компонента cmp_math
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Ветки конфигурации
    pub branches: Vec<ConfigBranch<TMsg>>,
}
// ANCHOR: Config

/// Конфигурация ветки
pub struct ConfigBranch<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Функция получения значения из входящих сообщений
    pub fn_input: fn(&TMsg) -> Option<ValueTime>,

    /// Алгоритмы математической обработки
    pub algs: Vec<Algs<TMsg>>,

    /// Функция создания исходящего сообщения на основе значения
    pub fn_output: fn(&ValueTime) -> Option<TMsg>,
}
