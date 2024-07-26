#![cfg(feature = "cmp_plc")]

use serde::Serialize;

use rsiot::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};

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
    fn logic(input: &I, stat: &mut S, _system_data: &FbSystemData) -> Q {
        println!("in fb2");

        stat.fb1_inst.call(fb1_example::I { counter: 1 });

        Q {
            out_counter: input.counter * 2,
        }
    }
}

pub type _FunctionBlock = FunctionBlockBase<I, Q, S>;
