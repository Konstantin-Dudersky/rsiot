//! Блок реализации машины состояния

mod i;
mod logic;
mod q;
mod s;

pub use i::*;
pub use q::*;
pub use s::*;
use serde::Serialize;

use crate::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};

/// Блок реализации машины состояния
pub type FB<TState> = FunctionBlockBase<I<TState>, Q<TState>, S<TState>>;

impl<TState> IFunctionBlock<I<TState>, Q<TState>, S<TState>>
    for FunctionBlockBase<I<TState>, Q<TState>, S<TState>>
where
    TState: Copy + Default + PartialEq + Serialize,
{
    fn logic(
        input: &mut I<TState>,
        stat: &mut S<TState>,
        _system_data: &FbSystemData,
    ) -> Q<TState> {
        logic::logic(input, stat)
    }
}
