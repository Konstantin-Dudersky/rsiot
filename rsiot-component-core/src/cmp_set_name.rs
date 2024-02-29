use tracing::info;

use rsiot_messages_core::{MsgDataBound, MsgTrace};

use crate::{CmpInput, CmpOutput};

pub fn cmp_set_component_name<TMsg>(
    input: &mut CmpInput<TMsg>,
    output: &mut CmpOutput<TMsg>,
    name: &str,
) where
    TMsg: MsgDataBound,
{
    info!("Component start: {}", name);
    let id = MsgTrace::generate_uuid();
    input.set_component_id(name, id);
    output.set_component_id(name, id);
}

pub fn cmp_set_session_name<TMsg>(
    input: &mut CmpInput<TMsg>,
    output: &mut CmpOutput<TMsg>,
    name: &str,
) where
    TMsg: MsgDataBound,
{
    info!("Session start: {}", name);
    let id = MsgTrace::generate_uuid();
    input.set_session_id(name, id);
    output.set_session_id(name, id);
}
