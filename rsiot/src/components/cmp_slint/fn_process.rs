use std::time::Duration;

use slint::SharedString;
use slint_interpreter::Value;
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, Result};

pub async fn fn_process<TMsg>(config: Config<TMsg>, input: CmpInOut<TMsg>) -> Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut counter = 0;
    loop {
        let lock = config.instance.lock().await;
        counter += 1;
        lock.upgrade_in_event_loop(move |handle| {
            let s = format!("{}", counter);
            let value = Value::String(SharedString::from(s));
            handle.set_property("text_content", value).unwrap();
        })
        .unwrap();
        sleep(Duration::from_secs(2)).await;
    }
}
