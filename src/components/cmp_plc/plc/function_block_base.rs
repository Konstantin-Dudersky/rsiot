//! Функциональный блок

use serde::{Deserialize, Serialize};

/// Функциональный блок
#[derive(Clone, Default, Deserialize, Serialize)]
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
    /// Создание экземпляря функционального блока со значениями по-умолчанию
    pub fn new() -> Self {
        Self {
            fb_system_data: FbSystemData { first_call: true },
            ..Default::default()
        }
    }

    pub(crate) fn new_with_restore_stat(self, stat: S) -> Self {
        Self {
            stat,
            fb_system_data: FbSystemData { first_call: true },
            ..Default::default()
        }
    }

    /// Вызов функционального блока
    pub fn call(&mut self, input: &mut I) -> Q {
        self.output = FunctionBlockBase::logic(input, &mut self.stat, &self.fb_system_data);
        self.input = input.clone();
        self.fb_system_data.first_call = false;
        self.output.clone()
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
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct FbSystemData {
    /// true - первый вызов функционального блока
    pub first_call: bool,
}
