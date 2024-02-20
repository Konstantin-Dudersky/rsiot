use std::fmt::Debug;

use serde::Serialize;

use rsiot_messages_core::message_v2::MsgSource;

use crate::{CmpInput, CmpOutput};

pub fn cmp_set_component_id<TMsg>(
    input: &mut CmpInput<TMsg>,
    output: &mut CmpOutput<TMsg>,
    component_name: &str,
) where
    TMsg: Clone + Debug + Serialize,
{
    let component_id = MsgSource::generate_uuid();
    input.set_component_id(component_name, component_id);
    output.set_component_id(component_name, component_id);
}
