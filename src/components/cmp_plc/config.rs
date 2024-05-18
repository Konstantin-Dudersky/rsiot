use std::time::Duration;

use serde::Serialize;

use crate::message::{Message, MsgDataBound};

use super::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

/// Конфигурация компонента ПЛК
#[derive(Clone)]
pub struct Config<TMsg, I, Q, S>
where
    TMsg: MsgDataBound,
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
    pub fn_input: fn(&mut I, &Message<TMsg>) -> (),

    /// Функция преобразования выходной структуры ПЛК в исходящие сообщения.
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// fn_output: |output: &fb_main::Q| vec![]
    /// ```
    pub fn_output: fn(&Q) -> Vec<Message<TMsg>>,

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

    /// Настройки сохранения состояния и восттановления при запуске
    pub retention: Option<ConfigRetention<TMsg, I, Q, S>>,
}

/// Настройка сохранения и восстановления области Static
#[derive(Clone)]
pub struct ConfigRetention<TMsg, I, Q, S>
where
    TMsg: MsgDataBound,
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
{
    pub save_period: Duration,
    pub fn_export: fn(&I, &Q, &S) -> Option<Vec<Message<TMsg>>>,
    pub fn_import_static: fn(&Message<TMsg>) -> anyhow::Result<Option<S>>,
    pub restore_timeout: Duration,
}

pub enum ConfigRetentionRestoreResult<S>
where
    S: Clone + Default + Serialize,
{
    NoRestoreData,
    RestoreDeserializationError,
    RestoreData(S),
}
