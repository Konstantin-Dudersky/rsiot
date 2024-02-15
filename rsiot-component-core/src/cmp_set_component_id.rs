use rsiot_messages_core::IMessage;

use crate::{CmpInput, CmpOutput};

pub fn cmp_set_component_id<TMsg>(
    input: &mut CmpInput<TMsg>,
    output: &mut CmpOutput<TMsg>,
    component_name: &str,
) where
    TMsg: IMessage,
{
    let component_id = input.set_component_name(component_name);
    output.set_component_id(component_id);
}
