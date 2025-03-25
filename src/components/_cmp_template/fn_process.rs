use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(_config: Config<TMsg>, _msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    unimplemented!()
}
