use std::sync::Arc;

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{super::RsiotI2cDriverBase, state::State};

pub struct PCF8575<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: u8,
    pub pins: Vec<super::PCF8575PinMode<TMsg>>,
}

impl<TMsg> PCF8575<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn fn_process(
        &self,
        in_out: CmpInOut<TMsg>,
        driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    ) {
        let state = State::new();
        let task_set: JoinSet<()> = JoinSet::new();
    }
}

async fn task_output<TMsg>(
    mut in_out: CmpInOut<TMsg>,
    fn_input: fn(Message<TMsg>) -> Option<bool>,
    state: Arc<Mutex<State>>,
    driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    address: u8,
    pin: u8,
) where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let value = fn_input(msg);
        let Some(value) = value else { continue };
        let state_bytes = {
            let mut state = state.lock().await;
            match value {
                true => state.set_output_high(pin),
                false => state.set_output_low(pin),
            }
            state.to_bytes()
        };
        {
            let mut driver = driver.lock().await;
            driver.write_read(address, &state_bytes, 2).await;
        }
    }
}
