use rsiot::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};
use serde::{Deserialize, Serialize};

pub type FB = FunctionBlockBase<I, Q, S>;

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &mut I, stat: &mut S, system_data: &FbSystemData) -> Q {
        logic(input, stat, system_data)
    }
}

/// Input ------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct I {}

/// Output -----------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Q {}

/// Static -----------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S {}

/// Логика -----------------------------------------------------------------------------------------
fn logic(_input: &mut I, _stat: &mut S, _system_data: &FbSystemData) -> Q {
    Q {}
}
