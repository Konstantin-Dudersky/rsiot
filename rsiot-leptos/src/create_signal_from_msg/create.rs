use leptos::*;
use rsiot_messages_core::{IMessage, IMsgContentValue, MsgContent};

use crate::GlobalState;

use super::Config;

pub fn create<TMsg, TValue>(
    config: Config<TMsg, TValue>,
) -> (ReadSignal<MsgContent<TValue>>, WriteSignal<TValue>)
where
    TValue: Clone + std::fmt::Debug + Default + IMsgContentValue + 'static,
    TMsg: IMessage + 'static,
{
    let gs = use_context::<GlobalState<TMsg>>().unwrap();

    let default_content = (config.fn_input)(&config.default).unwrap();
    let (input, input_set) = create_signal(default_content.clone());
    let (output, output_set) = create_signal(TValue::default());

    let cache = gs.cache.clone();
    {
        let lock = cache.blocking_read();
        let key = config.default.key();
        let msg = lock.get(&key);
        if let Some(msg) = msg {
            let content = (config.fn_input)(&msg).unwrap();
            input_set.set(content)
        }
    }

    let gs_clone = gs.clone();
    create_effect(move |_| {
        let msg = match gs_clone.input.get() {
            Some(msg) => msg,
            None => return (),
        };
        let msg = (config.fn_input)(&msg);
        if let Some(msg) = msg {
            input_set.set(msg)
        }
    });

    let gs_clone = gs.clone();
    create_effect(move |prev_value| {
        let output = output.get();
        if prev_value.is_none() {
            return ();
        }
        let msg = (config.fn_output)(output);
        let msg = match msg {
            Some(msg) => msg,
            None => return (),
        };
        gs_clone.output.set(Some(msg));
        ()
    });

    (input, output_set)
}
