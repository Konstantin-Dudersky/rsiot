//! Шаблон для нового функционального блока

use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    pub counter_u16_0_100: u16,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    pub counter_u16_0_100: u16,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(&mut self) -> Q {
        self.stat.counter_u16_0_100 += 1;
        if self.stat.counter_u16_0_100 > 100 {
            self.stat.counter_u16_0_100 = 0;
        }
        Q {
            counter_u16_0_100: self.stat.counter_u16_0_100,
        }
    }
}

#[allow(dead_code)]
pub type FB = FunctionBlockBase<I, Q, S>;
