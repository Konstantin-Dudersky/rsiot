use tracing::info;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::super::shared_state::TSharedState;

pub struct CmpPlcInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: CmpInOut<TMsg>,
    pub shared_state: TSharedState<TMsg>,
    pub fn_input: fn(&Message<TMsg>) -> Option<String>,
}

impl<TMsg> CmpPlcInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv_input().await {
            info!("Message!");
            let data = (self.fn_input)(&msg);

            let mut shared_state = self.shared_state.lock().await;
            let Some(data) = data else { continue };
            shared_state.cmp_plc_input = data;
        }

        Err(super::Error::TaskEndCmpPlcInput)
    }
}
