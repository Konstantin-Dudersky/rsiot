use std::time::Duration;

use crate::{
    executor::MsgBusOutput,
    message::{MsgDataBound, ValueTime},
};

use super::{
    AlgFnOutputMsgbus, AlgInput, AlgOutput, Error, Indicators, OutputValue,
    calculation::Calculation,
};

pub struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: AlgInput,
    pub output: AlgOutput,
    pub output_msgbus: MsgBusOutput<TMsg>,
    pub time_window: Duration,
    pub indicators: Indicators,
    pub fn_output_msgbus: AlgFnOutputMsgbus<TMsg, OutputValue>,
}

impl<TMsg> Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut calculation = Calculation::new(self.time_window, self.indicators);

        while let Some(input_value) = self.input.recv().await {
            let ov = calculation.step(input_value);
            let Some(output_value) = ov else { continue };

            let msg = (self.fn_output_msgbus)(&output_value);
            if let Some(msg) = msg {
                self.output_msgbus
                    .send(msg.to_message())
                    .await
                    .map_err(|_| Error::SendToMsgbus)?;
            }

            self.output
                .send(ValueTime {
                    value: output_value.value,
                    time: output_value.time,
                })
                .await
                .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("_alg_template")))?;
        }

        Err(Error::AlgTaskUnexpectedEnd(String::from("_alg_template")))
    }
}
