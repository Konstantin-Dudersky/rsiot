//! Шаблон для нового функционального блока

use serde::{Deserialize, Serialize};

use crate::components::cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

pub type FB = FunctionBlockBase<I, Q, S>;

// Input -------------------------------------------------------------------------------------------

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {}

// Output ------------------------------------------------------------------------------------------

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {}

// Stat --------------------------------------------------------------------------------------------

#[derive(Clone, Default, Deserialize, Serialize)]
struct S {}

// Logic -------------------------------------------------------------------------------------------

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(_input: &I, _stat: &mut S) -> Q {
        Q {}
    }
}
