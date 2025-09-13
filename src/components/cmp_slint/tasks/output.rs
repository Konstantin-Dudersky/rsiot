use slint::ComponentHandle;
use tokio::sync::mpsc;

use crate::message::{Message, MsgDataBound};

use super::super::{Error, OutputSender, Result, SlintWindow, config::FnOutput};

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

        let output_sender = OutputSender::new(&ch_tx);

        self.slint_window
            .upgrade_in_event_loop(move |h| (self.fn_output)(h, output_sender))?;

        while let Some(msg) = ch_rx.recv().await {
            let msg = Message::new_custom(msg);
            self.output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpsc)?;
        }

        Err(Error::TaskOutput)
    }
}
