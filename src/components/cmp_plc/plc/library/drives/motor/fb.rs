//! Шаблон для нового функционального блока

use serde::{Deserialize, Serialize};

use crate::components::cmp_plc::plc::function_block_base::{FunctionBlockBase, IFunctionBlock};

pub type FB = FunctionBlockBase<I, Q, S>;

// Input -------------------------------------------------------------------------------------------

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct I {
    pub auto_mode: bool,
    pub man_mode: bool,
}

// Output ------------------------------------------------------------------------------------------

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Q {
    pub status: super::messages::Status,
}

// Stat --------------------------------------------------------------------------------------------

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct S {
    mode: Mode,
}

// Logic -------------------------------------------------------------------------------------------

impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
    fn logic(input: &I, stat: &mut S) -> Q {
        // Выбор режима
        if input.auto_mode {
            stat.mode = Mode::Auto;
        } else if input.man_mode {
            stat.mode = Mode::Manual;
        }

        Q {
            status: super::messages::Status {
                man_act: stat.mode == Mode::Auto,
                aut_act: stat.mode == Mode::Manual,
            },
        }
    }
}

// other -------------------------------------------------------------------------------------------

#[derive(Clone, Default, PartialEq, Deserialize, Serialize)]
enum Mode {
    Auto,
    #[default]
    Manual,
}
