use slint::ComponentHandle;
use tokio::sync::mpsc;

use crate::message::{Message, MsgDataBound};

use super::super::{config::FnOutput, Error, Result, SlintWindow};

pub struct Output<TMsg, TMainWindow>
where
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    pub output: mpsc::Sender<Message<TMsg>>,
    pub slint_window: SlintWindow<TMainWindow>,
    pub fn_output: FnOutput<TMsg, TMainWindow>,
}

impl<TMsg, TMainWindow> Output<TMsg, TMainWindow>
where
    TMsg: 'static + MsgDataBound,
    TMainWindow: 'static + ComponentHandle,
{
    // TODO - если fn_output пустой, сваливается в ошибку
    pub async fn spawn(self) -> Result<()> {
        let (ch_tx, mut ch_rx) = mpsc::channel(1000);

        self.slint_window
            .upgrade_in_event_loop(move |h| (self.fn_output)(h, ch_tx))?;

        while let Some(msg) = ch_rx.recv().await {
            self.output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpsc)?;
        }

        Err(Error::TaskOutput)
    }
}
