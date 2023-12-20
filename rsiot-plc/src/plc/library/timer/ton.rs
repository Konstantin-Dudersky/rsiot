use serde::Serialize;

use crate::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};
use crate::plc::{library::edge_detect::rising_edge, types};

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
    fn logic(&mut self) -> Q {
        if self
            .stat
            .input_rising_edge
            .call(rising_edge::VarInput {
                i: self.input.input,
            })
            .q
        {
            self.stat.delay = types::TimeInstant::now();
        }

        Q {
            output: self.stat.delay.elapsed() >= self.input.preset_time,
            elapsed_time: self.stat.delay.elapsed(),
        }
    }
}

pub type FB = FunctionBlockBase<I, Q, S>;
