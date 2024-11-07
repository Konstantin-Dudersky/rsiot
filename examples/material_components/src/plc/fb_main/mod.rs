//! motor

mod i;
mod logic;
mod q;
mod s;

pub use i::*;
pub use q::*;
pub use s::*;

use rsiot::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};

use logic::logic;

/// motor
pub type FB = FunctionBlockBase<I, Q, S>;

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &mut I, stat: &mut S, system_data: &FbSystemData) -> Q {
        logic(input, stat, system_data)
    }
}
