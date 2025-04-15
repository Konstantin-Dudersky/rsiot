//! Функциональный блок

use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// Функциональный блок
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    /// Входные данные
    pub input: I,
    /// Выходные данные
    pub output: Q,
    /// Статичные данные - сохраняются между вызовами
    pub stat: S,
    /// Системные данные функционального блока
    fb_system_data: FbSystemData,
}

impl<I, Q, S> FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    /// Создание экземпляра функционального блока со значениями по-умолчанию
    pub fn new(period: Duration) -> Self {
        Self {
            fb_system_data: FbSystemData {
                first_call: true,
                period,
                last_call_time: SystemTime::now(),
            },
            ..Default::default()
        }
    }

    /// Создание экземпляра функционального блока с восстановленными значениями области stat
    pub(crate) fn new_with_restore_stat(self, stat: S, period: Duration) -> Self {
        Self {
            stat,
            fb_system_data: FbSystemData {
                first_call: true,
                period,
                last_call_time: SystemTime::now(),
            },
            ..Default::default()
        }
    }

    /// Вызов функционального блока
    pub fn call(&mut self, input: &mut I, period: Duration) -> Q {
        self.fb_system_data.period = period;
        // TODO - замерять фактический период вызова функционального блока, а не передавать
        // константу
        self.output = FunctionBlockBase::logic(input, &mut self.stat, &self.fb_system_data);
        self.input = input.clone();
        self.fb_system_data.first_call = false;
        self.output.clone()
    }

    /// Период вызова блока
    pub fn get_period(&self) -> Duration {
        self.fb_system_data.period
    }
}

/// Трейт для логики выполнения блока
pub trait IFunctionBlock<I, Q, S> {
    /// Основная логика выполнения блока
    ///
    /// Нужно переопределить для своего функционального блока.
    /// Вызывать самому не нужно, вызывается функцией `call`
    ///
    /// TODO: рассмотреть возможность добавления аргумента fn_output, чтобы блок самостоятельно
    /// мог генерировать исходящие сообщения
    fn logic(input: &mut I, stat: &mut S, fb_system_data: &FbSystemData) -> Q;
}

/// Системные данные функционального блока
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FbSystemData {
    /// true - первый вызов функционального блока
    pub first_call: bool,

    /// Период вызова блока
    pub period: Duration,

    /// Время последнего вызова.
    ///
    /// TODO - если нет ошибок компиляции в разных таргетах, сделать на основе этого поля
    /// определение периодичности вызовов и убрать ручное задание period
    pub last_call_time: SystemTime,
}

impl Default for FbSystemData {
    fn default() -> Self {
        Self {
            first_call: true,
            period: Duration::from_millis(100),
            last_call_time: SystemTime::now(),
        }
    }
}
