use std::time::Duration;

use serde::Serialize;

use rsiot_messages_core::message_v2::MsgContentBound;

use crate::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

/// Конфигурация компонента ПЛК
///
/// # Шаблон функционального блока
///
#[doc = include_str!("./template.rs")]
///
#[derive(Clone)]
pub struct Config<TMessage, I, Q, S>
where
    TMessage: MsgContentBound,
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    /// Функция преобразования входящих сообщений во входную структуру ПЛК.
    ///
    /// # Примеры
    ///
    /// ```rust
    /// fn_input: |input: &mut fb_main::I, msg: &TMessage| match msg {}
    /// ```
    pub fn_input: fn(&mut I, &TMessage) -> (),

    /// Функция преобразования выходной структуры ПЛК в исходящие сообщения.
    ///
    /// # Примеры
    ///
    /// ```rust
    /// fn_output: |output: &fb_main::Q| vec![]
    /// ```
    pub fn_output: fn(&Q) -> Vec<TMessage>,

    /// Главный функциональный блок ПЛК
    ///
    /// # Примеры
    ///
    /// ```rust
    /// fb_main: fb_main::FB::new()
    /// ```
    pub fb_main: FunctionBlockBase<I, Q, S>,

    /// Периодичность выполнения логики ПЛК
    ///
    /// # Примеры
    ///
    /// ```rust
    /// period: Duration::from_millis(100)
    /// ```
    pub period: Duration,
}
