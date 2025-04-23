use std::fmt::Debug;

use crate::message::MsgDataBound;

use super::Algs;

/// Конфигурация компонента cmp_math
pub struct Config<TMsg, TIntMsg>
where
    TMsg: MsgDataBound,
    TIntMsg: IntMsgBound,
{
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(TMsg) -> Option<TIntMsg>,

    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: fn(TIntMsg) -> Option<Vec<TMsg>>,

    /// Алгоритмы математической обработки
    pub algs: Vec<Algs<TIntMsg>>,
}

/// Типаж для внутренних сообщений
pub trait IntMsgBound: Clone + Copy + Debug + Send + Sync {}
