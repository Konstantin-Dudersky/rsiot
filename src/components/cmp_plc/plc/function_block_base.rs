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
            ..Default::default()
        }
    }

    pub(crate) fn new_with_restore_stat(self, stat: S) -> Self {
        Self {
            stat,
            ..Default::default()
        }
    }

    /// Вызов функционального блока
    pub fn call(&mut self, input: I) -> Q {
        self.input = input;
        self.output = FunctionBlockBase::logic(&self.input, &mut self.stat);
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
    fn logic(input: &I, stat: &mut S) -> Q;
}
