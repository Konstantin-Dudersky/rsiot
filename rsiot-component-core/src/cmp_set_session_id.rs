use rsiot_messages_core::message_v2::{MsgDataBound, MsgSource};

use crate::{CmpInput, CmpOutput};

pub fn cmp_set_session_id<TMsg>(
    input: &mut CmpInput<TMsg>,
    output: &mut CmpOutput<TMsg>,
    component_name: &str,
) where
    TMsg: MsgDataBound,
{
    let component_id = MsgSource::generate_uuid();
    input.set_session_id(component_name, component_id);
    output.set_session_id(component_name, component_id);
}
