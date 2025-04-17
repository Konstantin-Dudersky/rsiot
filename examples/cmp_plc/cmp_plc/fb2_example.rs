#![cfg(feature = "cmp_plc")]

use serde::Serialize;

use rsiot::components::cmp_plc::plc::{
    types::Resettable, FbSystemData, FunctionBlockBase, IFunctionBlock,
};

use super::fb1_example;

#[derive(Clone, Debug, Default, Serialize)]
pub struct I {
    pub counter: u32,
    pub resettable: Resettable<bool>,
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
    fn logic(input: &mut I, stat: &mut S, _system_data: &FbSystemData) -> Q {
        stat.fb1_inst.call(&mut fb1_example::I { counter: 1 });

        if input.resettable.get() {
            println!("Input resettable 1 TRUE");
        }

        Q {
            out_counter: input.counter * 2,
        }
    }
}

pub type FB = FunctionBlockBase<I, Q, S>;
