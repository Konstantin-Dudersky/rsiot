use tokio::{sync::mpsc, task::JoinHandle};
use tracing::warn;

use super::{Error, QueryStat};

pub struct WaitResult {
    pub input: mpsc::Receiver<JoinHandle<Result<QueryStat, Error>>>,
    pub fn_query_stat: fn(QueryStat),
}

impl WaitResult {
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(task) = self.input.recv().await {
            let res = task.await;

            match res {
                Ok(Ok(qs)) => (self.fn_query_stat)(qs),
                Ok(Err(err)) => warn!("Error sending to database: {}", err),
                Err(err) => warn!("Error sending to database: {}", err),
            }
        }
        Err(Error::TaskSendToDatabase)
    }
}
