use slint::ComponentHandle;
use tokio::sync::mpsc;

use crate::message::{Message, MsgDataBound};

use super::super::{config::FnInput, Error, Result, SlintWindow};

pub struct Input<TMsg, TMainWindow>
where
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub slint_window: SlintWindow<TMainWindow>,
    pub fn_input: FnInput<TMsg, TMainWindow>,
}

impl<TMsg, TMainWindow> Input<TMsg, TMainWindow>
where
    TMsg: 'static + MsgDataBound,
    TMainWindow: 'static + ComponentHandle,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(msg) = self.input.recv().await {
            self.slint_window
                .upgrade_in_event_loop(move |h| (self.fn_input)(msg, h))?;
        }

        Err(Error::TaskInput)
    }
}
