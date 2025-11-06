use std::time::Duration;

use crate::{
    executor::MsgBusOutput,
    message::{MsgDataBound, ValueTime},
};

use super::{
    AlgFnOutputMsgbus, AlgInput, AlgOutput, Error, Gamma, OutputValue, calculation::Calculation,
};

pub struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: AlgInput,
    pub output: AlgOutput,
    pub output_msgbus: MsgBusOutput<TMsg>,
    pub time_window: Duration,
    pub normalization_time: Duration,
    pub gamma: Gamma,
    pub fn_output_msgbus: AlgFnOutputMsgbus<TMsg, OutputValue>,
}

impl<TMsg> Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut calculation = Calculation::new(self.gamma, self.normalization_time);

        while let Some(vt) = self.input.recv().await {
            let out_value = calculation.step(vt, self.time_window);

            let msg = (self.fn_output_msgbus)(&out_value);
            if let Some(msg) = msg {
                self.output_msgbus
                    .send(msg.to_message())
                    .await
                    .map_err(|_| Error::SendToMsgbus)?;
            }

            self.output
                .send(ValueTime {
                    value: out_value.derivative,
                    time: out_value.time,
                })
                .await
                .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("derivative")))?;
        }

        let err = String::from("derivative");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}
