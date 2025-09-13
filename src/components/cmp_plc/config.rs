use std::time::Duration;

use serde::Serialize;

use crate::message::{Message, MsgDataBound};

use super::plc::{FunctionBlockBase, IFunctionBlock};

type TFnExport<TMsg, I, Q, S> = fn(&I, &Q, &S) -> Option<Vec<Message<TMsg>>>;

// ANCHOR: config_1

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
    pub fn_input: fn(&mut I, &TMsg) -> (),

    // ANCHOR: config_1
    // ANCHOR: config_2
    /// Функция преобразования выходной структуры ПЛК в исходящие сообщения.
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// fn_output: |output: &fb_main::Q| vec![]
    /// ```
    pub fn_output: fn(&Q) -> Vec<TMsg>,

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

    /// Настройки сохранения состояния и восстановления при запуске
    pub retention: Option<ConfigRetention<TMsg, I, Q, S>>,
}
// ANCHOR: config_2

impl<TMsg, I, Q, S> Default for Config<TMsg, I, Q, S>
where
    TMsg: MsgDataBound,
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    fn default() -> Self {
        Self {
            fn_cycle_init: |_| (),
            fn_input: |_, _| (),
            fn_output: |_| vec![],
            fb_main: FunctionBlockBase::new(),
            period: Duration::default(),
            retention: None,
        }
    }
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
    /// Периодичность сохранения текущего состояния
    pub save_period: Duration,

    /// Функция преобразования состояния ПЛК в исходящие сообщения
    pub fn_export: TFnExport<TMsg, I, Q, S>,

    /// Функция восстановления состояния из входящих сообщений
    pub fn_import_static: fn(&Message<TMsg>) -> anyhow::Result<Option<S>>,

    /// Таймаут восстановления
    ///
    /// Если в течение заданного времени не будет получено сообщение с данными для восстановления,
    /// считаем что восттановление не удалось и запускаем ПЛК с дефолтным состоянием
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
