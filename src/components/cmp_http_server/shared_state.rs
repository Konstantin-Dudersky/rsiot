use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

#[derive(Clone)]
pub struct SharedState<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msg_bus: CmpInOut<TMsg>,
    pub config: Config<TMsg>,
    pub cmp_plc_input: String,
    pub cmp_plc_output: String,
    pub cmp_plc_static: String,
}

pub type TSharedState<TMsg> = Arc<Mutex<SharedState<TMsg>>>;
