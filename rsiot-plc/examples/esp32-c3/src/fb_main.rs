use serde::{Deserialize, Serialize};

use rsiot::cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    pub inject_u16: u16,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    pub inject_u16: u16,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    pub counter: u16,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(&mut self) -> Q {
        self.stat.counter += 1;

        Q {
            inject_u16: self.input.inject_u16 + self.stat.counter,
        }
    }
}

#[allow(dead_code)]
pub type FB = FunctionBlockBase<I, Q, S>;
