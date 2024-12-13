use leptos::prelude::*;
use tracing::{info, trace, warn};

use crate::message::MsgDataBound;

use super::{super::GlobalState, Config};

/// Функция создания сигнала на основе сообщений
pub fn create<TMsg, TValue>(
    config: Config<TMsg, TValue>,
) -> (ReadSignal<TValue>, WriteSignal<TValue>)
where
    TValue: Clone + std::fmt::Debug + Default + 'static + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    let gs = use_context::<GlobalState<TMsg>>().expect("No global state in create_signal_from_msg");

    let default_content = (config.fn_input)(&config.default).unwrap();
    let (input, input_set) = signal(default_content.clone());
    let (output, output_set) = signal(TValue::default());

    let cache = gs.cache.clone();
    {
        let lock = cache.blocking_read();
        let key = config.default.key;
        let msg = lock.get(&key);
        if let Some(msg) = msg {
            let content = (config.fn_input)(msg).unwrap();
            input_set.set(content)
        }
    }

    let gs_clone = gs.clone();
    Effect::new(move |_| {
        let msg = match gs_clone.input.get() {
            Some(msg) => msg,
            None => return,
        };
        trace!("create_signal_from_msg: {}", msg.key);
        let msg = (config.fn_input)(&msg);
        if let Some(msg) = msg {
            input_set.set(msg)
        }
    });

    let gs_clone = gs.clone();
    Effect::new(move |prev_value: Option<()>| {
        let output = output.get();
        if prev_value.is_none() {
            return;
        }
        let msg = (config.fn_output)(output);
        let msg = match msg {
            Some(msg) => msg,
            None => return,
        };
        gs_clone.output.set(Some(msg));
    });

    (input, output_set)
}
