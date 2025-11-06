use std::time::Duration;

use tokio::sync::broadcast;

use crate::message::{Message, MsgDataBound};

use super::{ComponentError, TokioRuntimeMetrics};

pub struct TaskRuntimeMetrics<TMsg>
where
    TMsg: MsgDataBound,
{
    pub output: broadcast::Sender<Message<TMsg>>,
    pub period: Duration,
    pub fn_tokio_metrics: super::component_executor::FnTokioMetrics<TMsg>,
}

impl<TMsg> TaskRuntimeMetrics<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), ComponentError> {
        let handle = tokio::runtime::Handle::current();
        let runtime_monitor = tokio_metrics::RuntimeMonitor::new(&handle);
        for metrics in runtime_monitor.intervals() {
            tokio::time::sleep(self.period).await;
            let metrics: TokioRuntimeMetrics = metrics.into();
            let msg = (self.fn_tokio_metrics)(metrics);
            let Some(msg) = msg else { continue };
            let msg = Message::new_custom(msg);
            self.output.send(msg).map_err(|err| {
                let err =
                    format!("Internal task of ComponentExecutor: send to channel error, {err:?}");
                ComponentError::Initialization(err)
            })?;
        }
        Ok(())
    }
}
