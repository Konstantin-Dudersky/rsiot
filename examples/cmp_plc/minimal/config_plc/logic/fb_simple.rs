use rsiot::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};
use serde::{Deserialize, Serialize};

/// Входная структура
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct I {}

/// Область памяти output
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Q {}

/// Область памяти stat
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S {}

fn logic(_input: &mut I, _stat: &mut S) -> Q {
    Q {}
}

pub type FB = FunctionBlockBase<I, Q, S>;

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &mut I, stat: &mut S, _system_data: &FbSystemData) -> Q {
        logic(input, stat)
    }
}
