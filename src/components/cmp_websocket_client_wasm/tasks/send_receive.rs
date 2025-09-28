use std::time::Duration;

use futures::StreamExt;
use gloo::net::websocket::futures::WebSocket;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::{info, warn};
use url::Url;

use crate::executor::{join_set_spawn, sleep};

use super::{Error, Receive, Send};

pub struct SendReceive {
    pub url: String,
    pub ch_rx_input_to_send: broadcast::Receiver<Vec<u8>>,
    pub ch_tx_receive_to_output: mpsc::Sender<Vec<u8>>,
    pub ch_tx_connection_state: mpsc::Sender<bool>,
}

impl SendReceive {
    pub async fn spawn(self) -> Result<(), Error> {
        let url = Url::parse(&self.url).map_err(Error::BadUrl)?;
        let url = url.to_string();

        loop {
            let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

            let ws = WebSocket::open(&url).map_err(|e| Error::SetupConnection(e.to_string()));

            let ws_stream = match ws {
                Ok(v) => v,
                Err(e) => {
                    warn!("Failed to connect to websocket server: {}", e);
                    self.send_connection_state(false).await?;
                    sleep(Duration::from_millis(2_000)).await;
                    continue;
                }
            };

            info!("Connection to websocket server established");

            self.send_connection_state(true).await?;

            let (websocket_write, websocket_read) = ws_stream.split();

            // Задача отправки текста на сервер
            let task = Send {
                input: self.ch_rx_input_to_send.resubscribe(),
                websocket_write,
            };
            join_set_spawn(&mut task_set, "cmp_websocket_client | send", task.spawn());

            // Задача получения текста из сервера
            let task = Receive {
                websocket_read,
                output: self.ch_tx_receive_to_output.clone(),
            };
            join_set_spawn(
                &mut task_set,
                "cmp_websocket_client | receive",
                task.spawn(),
            );

            while let Some(task_result) = task_set.join_next().await {
                warn!("Task completed with result: {:?}", task_result);
                self.send_connection_state(false).await?;
                task_set.shutdown().await;
            }

            sleep(Duration::from_millis(2_000)).await;
        }
    }

    async fn send_connection_state(&self, state: bool) -> Result<(), Error> {
        self.ch_tx_connection_state
            .send(state)
            .await
            .map_err(|_| Error::TokioSyncMpscSend)
    }
}
