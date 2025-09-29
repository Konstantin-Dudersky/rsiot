use crate::{
    executor::{CmpInOut, MsgBusInput, MsgBusOutput},
    message::MsgDataBound,
};

use super::Config;

pub async fn fn_process<TMsg>(
    _config: Config<TMsg>,
    _input: MsgBusInput<TMsg>,
    _output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    unimplemented!()
}
