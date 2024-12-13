use std::time::Duration;

use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    sleep(config.delay).await;

    let mut msgs = vec![];

    let cache = msg_bus.cache.read().await;
    for msg in config.msgs {
        if cache.contains_key(&msg.key) {
            continue;
        }
        msgs.push(msg);
    }
    drop(cache);

    sleep(Duration::MAX).await;
    Ok(())
}
