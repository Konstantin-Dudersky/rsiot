use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::super::{shared_state::TSharedState, ConfigCmpPlcData};

pub struct CmpPlcData<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: CmpInOut<TMsg>,
    pub shared_state: TSharedState<TMsg>,
    pub fn_input: fn(&Message<TMsg>) -> ConfigCmpPlcData,
}

impl<TMsg> CmpPlcData<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(msg) = self.input.recv_input().await {
            let data = (self.fn_input)(&msg);

            // Выходим из блока, чтобы не блокировать SharedState
            if matches!(data, ConfigCmpPlcData::NoData) {
                continue;
            }

            let mut shared_state = self.shared_state.lock().await;
            match data {
                ConfigCmpPlcData::NoData => continue,
                ConfigCmpPlcData::Input(data) => shared_state.cmp_plc_input = data,
                ConfigCmpPlcData::Output(data) => shared_state.cmp_plc_output = data,
                ConfigCmpPlcData::Static(data) => shared_state.cmp_plc_static = data,
            }
        }

        Err(super::Error::TaskEndCmpPlcInput)
    }
}
