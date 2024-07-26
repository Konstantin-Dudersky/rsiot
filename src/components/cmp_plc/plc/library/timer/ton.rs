//! Таймер TON

use serde::Serialize;

use crate::components::cmp_plc::plc::FbSystemData;

use super::super::super::function_block_base::{FunctionBlockBase, IFunctionBlock};
use super::super::super::{library::edge_detect::rising_edge, types};

/// Входные данные
#[derive(Clone, Default, Serialize)]
pub struct I {
    /// Входной сигнал для контроля
    pub input: bool,
    /// Задание времени
    pub preset_time: types::TimeDuration,
}

/// Выходные данные
#[derive(Clone, Default, Serialize)]
pub struct Q {
    /// Устанавливается в true, когда таймер насчитал время
    pub output: bool,
    /// Время работы таймера
    pub elapsed_time: types::TimeDuration,
}

/// Статичные данные
#[derive(Clone, Default, Serialize)]
pub struct S {
    input_rising_edge: rising_edge::FB,
    delay: types::TimeInstant,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S, _system_data: &FbSystemData) -> Q {
        if stat
            .input_rising_edge
            .call(rising_edge::I { i: input.input })
            .q
        {
            stat.delay = types::TimeInstant::now();
        }

        Q {
            output: stat.delay.elapsed() >= input.preset_time,
            elapsed_time: stat.delay.elapsed(),
        }
    }
}

/// Таймер TON
pub type FB = FunctionBlockBase<I, Q, S>;
