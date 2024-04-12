use std::time::Duration;

use serde::Serialize;

use crate::message::{Message, MsgDataBound};

use super::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

/// Конфигурация компонента ПЛК
#[derive(Clone)]
pub struct Config<TMessage, I, Q, S>
where
    TMessage: MsgDataBound,
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    /// Функция инициализации входной структуры в начале цикла ПЛК
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// fn_cycle_init: |_input: &mut fb_main::I| {}
    /// ```
    pub fn_cycle_init: fn(&mut I) -> (),

    /// Функция преобразования входящих сообщений во входную структуру ПЛК.
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// fn_input: |input: &mut fb_main::I, msg: &Message<Custom>| match msg {}
    /// ```
    pub fn_input: fn(&mut I, &Message<TMessage>) -> (),

    /// Функция преобразования выходной структуры ПЛК в исходящие сообщения.
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// fn_output: |output: &fb_main::Q| vec![]
    /// ```
    pub fn_output: fn(&Q) -> Vec<Message<TMessage>>,

    /// Главный функциональный блок ПЛК
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// fb_main: fb_main::FB::new()
    /// ```
    pub fb_main: FunctionBlockBase<I, Q, S>,

    /// Периодичность выполнения логики ПЛК
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// period: Duration::from_millis(100)
    /// ```
    pub period: Duration,
}
