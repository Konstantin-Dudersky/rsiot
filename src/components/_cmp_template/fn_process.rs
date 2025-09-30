use crate::{executor::MsgBusLinker, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(
    _config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    msgbus_linker.close();
    unimplemented!()
}
