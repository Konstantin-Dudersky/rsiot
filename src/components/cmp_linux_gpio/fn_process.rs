use tokio::task::JoinSet;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, Error};

pub async fn fn_process<TMsg>(_config: Config<TMsg>, _msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    Ok(())
}
