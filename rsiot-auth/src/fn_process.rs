use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentError};
use rsiot_messages_core::MsgDataBound;

use crate::Config;

pub async fn fn_process<TMsg>(
    input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    config: Config<TMsg>,
    cache: Cache<TMsg>,
) -> std::result::Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    Ok(())
}
