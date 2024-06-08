//! Выбор уставки из двух источников

mod i;
mod logic;
mod q;
mod s;

pub use i::*;
pub use q::*;
pub use s::*;

use crate::components::cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

use logic::logic;

/// motor
#[allow(dead_code)]
pub type FB = FunctionBlockBase<I, Q, S>;

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S) -> Q {
        logic(input, stat)
    }
}
