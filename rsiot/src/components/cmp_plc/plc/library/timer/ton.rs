use serde::Serialize;

use super::super::super::function_block_base::{FunctionBlockBase, IFunctionBlock};
use super::super::super::{library::edge_detect::rising_edge, types};

#[derive(Clone, Default, Serialize)]
pub struct I {
    pub input: bool,
    pub preset_time: types::TimeDuration,
}

#[derive(Clone, Default, Serialize)]
pub struct Q {
    pub output: bool,
    pub elapsed_time: types::TimeDuration,
}

#[derive(Clone, Default, Serialize)]
pub struct S {
    input_rising_edge: rising_edge::FunctionBlock,
    delay: types::TimeInstant,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S) -> Q {
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

pub type FB = FunctionBlockBase<I, Q, S>;
