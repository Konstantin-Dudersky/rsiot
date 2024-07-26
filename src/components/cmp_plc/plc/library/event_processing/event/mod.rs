//! Функциональный блок события

mod i;
mod logic;
mod q;
mod s;
#[cfg(test)]
mod tests;

pub use i::*;
pub use q::*;
pub use s::*;

use crate::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};

/// motor
pub type FB = FunctionBlockBase<I, Q, S>;

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S, _system_data: &FbSystemData) -> Q {
        logic::logic(input, stat)
    }
}
