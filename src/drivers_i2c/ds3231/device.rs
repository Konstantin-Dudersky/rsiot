use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, task::JoinSet};
use tracing::warn;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{
    super::{I2cSlaveAddress, RsiotI2cDriverBase},
    task_output::{OutputData, TaskOutput},
};

pub struct DS3231<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес. По-умолчанию 0x68
    pub address: I2cSlaveAddress,
    pub fn_output: fn(OutputData) -> Option<Vec<Message<TMsg>>>,
    pub in_out: CmpInOut<TMsg>,
}

impl<TMsg> DS3231<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    pub async fn fn_process(&self, driver: Arc<Mutex<impl RsiotI2cDriverBase + 'static>>) {
        loop {
            let mut task_set: JoinSet<Result<(), String>> = JoinSet::new();

            let task = TaskOutput {
                address: self.address,
                period: Duration::from_secs(5),
                driver: driver.clone(),
                fn_output: self.fn_output,
                in_out: self.in_out.clone(),
            };
            task_set.spawn(async move { task.spawn().await });

            while let Some(res) = task_set.join_next().await {
                warn!("{res:?}");
                task_set.shutdown().await;
            }
        }
    }
}
