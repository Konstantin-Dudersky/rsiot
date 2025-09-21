use serde::Serialize;

use rsiot::components::cmp_plc::plc::{
    FbSystemData, FunctionBlockBase, IFunctionBlock, library::timer::ton, types,
};

#[derive(Clone, Default, Serialize)]
pub struct I {
    pub counter: u16,
}

#[derive(Clone, Default, Serialize)]
pub struct Q {
    pub out_counter: u16,
}

#[derive(Clone, Default, Serialize)]
pub struct S {
    timer: ton::FB,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &mut I, stat: &mut S, _system_data: &FbSystemData) -> Q {
        let _ton_res = stat.timer.call(&mut ton::I {
            input: true,
            preset_time: types::TimeDuration::from_secs(10),
        });

        Q {
            out_counter: input.counter,
        }
    }
}

pub type FB = FunctionBlockBase<I, Q, S>;
