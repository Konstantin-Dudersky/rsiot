//! Шаблон для нового функционального блока

use serde::{Deserialize, Serialize};

use crate::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(_input: &I, _stat: &mut S) -> Q {
        Q {}
    }
}

#[allow(dead_code)]
pub type FB = FunctionBlockBase<I, Q, S>;
