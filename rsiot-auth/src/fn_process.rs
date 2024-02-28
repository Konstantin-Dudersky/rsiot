use rsiot_component_core::{Cache, CmpInput, CmpOutput};
use rsiot_messages_core::MsgDataBound;

use crate::Config;

pub async fn fn_process<TMsg>(
    input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    config: Config,
    cache: Cache<TMsg>,
) -> crate::Result<()>
where
    TMsg: MsgDataBound,
{
    loop {}
}
