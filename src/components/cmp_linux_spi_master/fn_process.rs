use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::Config;

pub async fn fn_process<TMsg, TService>(
    _config: Config<TMsg>,
    _msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    unimplemented!()
}
