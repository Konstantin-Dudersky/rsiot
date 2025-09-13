use std::time::Duration;
use time::OffsetDateTime;

use crate::message::ValueTime;

use super::{
    AlgInput, AlgOutput, Error, Gamma, IntMsgBound, OutputValue, Result, calculation::Calculation,
};

pub struct Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub input: AlgInput<TIntMsg>,
    pub output: AlgOutput<TIntMsg>,
    pub fn_input_value: fn(TIntMsg) -> Option<(f64, OffsetDateTime)>,
    pub fn_input_time_window: fn(TIntMsg) -> Option<Duration>,
    pub normalization_time: Duration,
    pub gamma: Gamma,
    pub fn_output: fn(OutputValue) -> TIntMsg,
}

impl<TIntMsg> Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        let mut time_window = Duration::default();
        let mut calculation = Calculation::new(self.gamma, self.normalization_time);

        while let Ok(input_int_msg) = self.input.recv().await {
            // Получаем новое значение окна времени
            if let Some(new_time_window) = (self.fn_input_time_window)(input_int_msg) {
                time_window = new_time_window;
            }

            // Получаем новое значение
            if let Some((value, time)) = (self.fn_input_value)(input_int_msg) {
                let new_value = ValueTime { value, time };

                let out_value = calculation.step(new_value, time_window);

                let output_int_msg = (self.fn_output)(out_value);
                self.output
                    .send(output_int_msg)
                    .await
                    .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("derivative")))?;
            };
        }

        let err = String::from("derivative");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}
