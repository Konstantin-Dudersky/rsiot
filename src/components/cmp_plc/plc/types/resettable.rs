//! Структура данных, которая сбрасывает свое значение на значение по-умолчанию после одного чтения

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// Структура данных, которая сбрасывает свое значение на значение по-умолчанию после одного чтения
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Resettable<T>
where
    T: Clone + Debug + Default + Serialize,
{
    value: T,
}

impl<T> Resettable<T>
where
    T: Clone + Debug + Default + Serialize,
{
    /// Создание новой структуры
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Получение данных
    pub fn get(&mut self) -> T {
        let value = self.value.clone();
        self.value = Default::default();
        value
    }
}
