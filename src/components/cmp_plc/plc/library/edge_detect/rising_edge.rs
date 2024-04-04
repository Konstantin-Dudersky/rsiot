//! Фронт сигнала 0 -> 1

use serde::{Deserialize, Serialize};

use super::super::super::function_block_base::{FunctionBlockBase, IFunctionBlock};

/// Входные данные
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    /// Сигнал, фронт которого нужно определить
    pub i: bool,
}

/// Выходные данные
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    /// Устанавливается в true на один цикл при обнаруженном фронте сигнала 0 -> 1
    pub q: bool,
}

/// Статичные данные
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    prev_i: bool,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S) -> Q {
        let rising_edge = input.i && !stat.prev_i;
        stat.prev_i = input.i;

        Q { q: rising_edge }
    }
}

/// Фронт сигнала 0 -> 1
pub type FB = FunctionBlockBase<I, Q, S>;
