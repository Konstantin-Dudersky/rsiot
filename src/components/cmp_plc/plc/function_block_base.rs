//! Функциональный блок

use serde::Serialize;

/// Функциональный блок
#[derive(Clone, Default, Serialize)]
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
    fn logic(input: &I, stat: &mut S) -> Q;
}
