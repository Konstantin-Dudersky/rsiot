use slint::ComponentHandle;

use crate::{executor::MsgBusInput, message::MsgDataBound};

use super::super::{Error, Result, SlintWindow, config::FnInput};

pub struct Input<TMsg, TMainWindow>
where
    TMsg: MsgDataBound,
    TMainWindow: ComponentHandle,
{
    pub input: MsgBusInput<TMsg>,
    pub slint_window: SlintWindow<TMainWindow>,
    pub fn_input: FnInput<TMsg, TMainWindow>,
}

impl<TMsg, TMainWindow> Input<TMsg, TMainWindow>
where
    TMsg: 'static + MsgDataBound,
    TMainWindow: 'static + ComponentHandle,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            self.slint_window
                .upgrade_in_event_loop(move |h| (self.fn_input)(msg, h))?;
        }

        Err(Error::TaskInput)
    }
}
