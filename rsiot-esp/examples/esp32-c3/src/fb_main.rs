use rgb::RGB8;
use serde::{Deserialize, Serialize};

use rsiot::components::cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    pub color: RGB8,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    pub counter: u16,
}

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(&mut self) -> Q {
        let color = match self.stat.counter {
            0..=10 => RGB8::new(0, 0, 0),
            11..=20 => RGB8::new(255, 0, 0),
            21..=30 => RGB8::new(0, 255, 0),
            31..=40 => RGB8::new(0, 0, 255),
            41..=50 => RGB8::new(255, 255, 0),
            51..=60 => RGB8::new(255, 0, 255),
            61..=70 => RGB8::new(0, 255, 255),
            71..=80 => RGB8::new(255, 255, 255),
            _ => RGB8::new(255, 0, 0),
        };

        self.stat.counter += 1;
        if self.stat.counter > 80 {
            self.stat.counter = 0;
        }

        Q { color }
    }
}

#[allow(dead_code)]
pub type FB = FunctionBlockBase<I, Q, S>;
