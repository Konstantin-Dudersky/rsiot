//! Функциональный блок

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;
#[cfg(target_arch = "wasm32")]
use web_time::SystemTime;

/// Функциональный блок
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    /// Входные данные
    pub i: I,
    /// Выходные данные
    pub q: Q,
    /// Статичные данные - сохраняются между вызовами
    pub s: S,

    calc_period_errors: usize,

    /// Время последнего вызова.
    ///
    /// TODO - если нет ошибок компиляции в разных таргетах, сделать на основе этого поля
    /// определение периодичности вызовов и убрать ручное задание period
    pub last_call_time: SystemTime,

    /// true - первый вызов функционального блока
    pub first_call: bool,
}

impl<I, Q, S> Default for FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    fn default() -> Self {
        Self {
            i: I::default(),
            q: Q::default(),
            s: S::default(),
            calc_period_errors: 0,
            last_call_time: SystemTime::now(),
            first_call: true,
        }
    }
}

impl<I, Q, S> FunctionBlockBase<I, Q, S>
where
    I: Clone + Default + Serialize,
    Q: Clone + Default + Serialize,
    S: Clone + Default + Serialize,
    Self: IFunctionBlock<I, Q, S>,
{
    /// Создание экземпляра функционального блока со значениями по-умолчанию
    pub fn new() -> Self {
        Self::default()
    }

    /// Создание экземпляра функционального блока с восстановленными значениями области stat
    pub(crate) fn new_with_restore_stat(self, stat: S) -> Self {
        Self {
            s: stat,
            ..Default::default()
        }
    }

    /// Вызов функционального блока
    pub fn call(&mut self, input: &mut I) -> Q {
        let now = SystemTime::now();

        let period = now.duration_since(self.last_call_time);
        self.last_call_time = now;

        let period = match period {
            Ok(v) => {
                if v >= Duration::from_millis(5000) {
                    Duration::from_millis(5000)
                } else {
                    v
                }
            }
            Err(_) => {
                self.calc_period_errors += 1;
                Duration::from_millis(0)
            }
        };

        let fb_system_data = FbSystemData {
            first_call: self.first_call,
            period,
        };
        // TODO - замерять фактический период вызова функционального блока, а не передавать
        // константу
        self.q = FunctionBlockBase::logic(input, &mut self.s, &fb_system_data);
        self.i = input.clone();
        self.first_call = false;
        self.q.clone()
    }
}

/// Трейт для логики выполнения блока
pub trait IFunctionBlock<I, Q, S> {
    /// Основная логика выполнения блока
    ///
    /// Нужно переопределить для своего функционального блока.
    /// Вызывать самому не нужно, вызывается функцией `call`
    fn logic(input: &mut I, stat: &mut S, fb_system_data: &FbSystemData) -> Q;
}

/// Системные данные функционального блока
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FbSystemData {
    /// true - первый вызов функционального блока
    pub first_call: bool,

    /// Период вызова блока
    pub period: Duration,
}

impl Default for FbSystemData {
    fn default() -> Self {
        Self {
            first_call: true,
            period: Duration::from_millis(100),
        }
    }
}
