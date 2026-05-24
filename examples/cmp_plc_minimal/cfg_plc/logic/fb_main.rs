use rsiot::components::cmp_plc::plc::{
    FbSystemData, FunctionBlockBase, IFunctionBlock, types::Resettable,
};
use serde::{Deserialize, Serialize};

pub type FB = FunctionBlockBase<I, Q, S>;

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &mut I, stat: &mut S, system_data: &FbSystemData) -> Q {
        logic(input, stat, system_data)
    }
}

/// Input ------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct I {
    pub counter: u32,
    pub resettabe_bit: Resettable<bool>,
}

/// Output -----------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Q {}

/// Static -----------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S {
    pub fb_simple: super::fb_simple::FB,
}

/// Логика -----------------------------------------------------------------------------------------
pub fn logic(_input: &mut I, _stat: &mut S, _system_data: &FbSystemData) -> Q {
    Q {}
}
