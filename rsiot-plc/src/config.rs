use std::time::Duration;

use serde::Serialize;

use rsiot_messages_core::IMessage;

use crate::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone)]
pub struct Config<TMessage, I, Q, S>
where
    TMessage: IMessage,
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    /// Функция преобразования входящих сообщений во входную структуру ПЛК.
    ///
    /// Шаблон:
    ///
    /// ```rust
    /// fn_input: |input: &mut fb_main::I, msg: &TMessage| match msg {}
    /// ```
    pub fn_input: fn(&mut I, &TMessage) -> (),

    /// Функция преобразования выходной структуры ПЛК в исходящие сообщения.
    ///
    /// Шаблон:
    ///
    /// ```rust
    /// fn_output: |output: &fb_main::Q| vec![]
    /// ```
    pub fn_output: fn(&Q) -> Vec<TMessage>,

    /// Главный функциональный блок ПЛК
    pub fb_main: FunctionBlockBase<I, Q, S>,

    /// Периодичность выполнения логики ПЛК
    ///
    /// Шаблон:
    ///
    /// ```rust
    /// period: Duration::from_millis(100)
    /// ```
    pub period: Duration,

    /// Размер внутренней очереди сообщений. По-умолчанию можно задать 100.
    pub buffer_size: usize,
}
