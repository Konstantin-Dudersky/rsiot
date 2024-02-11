use serde::{Deserialize, Serialize};

use crate::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    pub i: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    pub q: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    prev_i: bool,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S) -> Q {
        let rising_edge = input.i && !stat.prev_i;
        stat.prev_i = input.i;

        Q { q: rising_edge }
    }
}

pub type FunctionBlock = FunctionBlockBase<I, Q, S>;
