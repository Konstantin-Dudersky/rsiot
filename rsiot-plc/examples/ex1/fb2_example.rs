use serde::Serialize;

use crate::function_block::{FunctionBlockBase, IFunctionBlock};

use super::fb1_example;

#[derive(Clone, Default, Serialize)]
pub struct I {
    pub counter: u32,
}

#[derive(Clone, Default, Serialize)]
pub struct Q {
    pub out_counter: u32,
}

#[derive(Clone, Default, Serialize)]
pub struct S {
    pub internal_counter: u32,
    pub fb1_inst: fb1_example::FB,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(&mut self) -> Q {
        println!("in fb2");
        let mut internal_counter = self.stat.internal_counter;

        self.stat.fb1_inst.call(fb1_example::I { counter: 1 });

        Q {
            out_counter: self.input.counter * 2,
        }
    }
}

pub type FunctionBlock = FunctionBlockBase<I, Q, S>;
