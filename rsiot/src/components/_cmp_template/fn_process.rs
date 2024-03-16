use std::time::Duration;

use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(_config: Config<TMsg>, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    loop {
        sleep(Duration::from_secs(2)).await;
    }
    Ok(())
}
