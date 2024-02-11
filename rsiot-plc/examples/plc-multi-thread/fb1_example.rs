use serde::Serialize;

use rsiot_plc::plc::{
    function_block_base::{FunctionBlockBase, IFunctionBlock},
    library::timer::ton,
    types,
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
    fn logic(input: &I, stat: &mut S) -> Q {
        let ton_res = stat.timer.call(ton::I {
            input: true,
            preset_time: types::TimeDuration::from_secs(10),
        });

        println!(
            "in fb1, timer: {}, elapsed: {:?}",
            ton_res.output, ton_res.elapsed_time
        );

        Q {
            out_counter: input.counter,
        }
    }
}

pub type FB = FunctionBlockBase<I, Q, S>;
