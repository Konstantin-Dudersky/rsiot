use std::time::Duration;

use crate::{
    executor::MsgBusOutput,
    message::{MsgDataBound, ValueTime},
};

use super::{AlgFnOutputMsgbus, AlgInput, AlgOutput, EmaKind, Error, OutputValue};

pub struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: AlgInput,
    pub output: AlgOutput,
    pub output_msgbus: MsgBusOutput<TMsg>,
    pub kind: EmaKind,
    pub time_window: Duration,
    pub fn_output_msgbus: AlgFnOutputMsgbus<TMsg, OutputValue>,
}

impl<TMsg> Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut prev_value = ValueTime::default();
        let mut prev_ema = 0.0;
        let mut init = true;

        while let Some(input_value) = self.input.recv().await {
            // Пропускаем первое значение
            if init {
                prev_value = input_value;
                init = false;
                continue;
            }

            let ema = EmaCalc {
                kind: self.kind,
                prev_value,
                prev_ema,
                value: input_value,
                time_window: self.time_window,
            }
            .calc();
            prev_value = input_value;
            prev_ema = ema;

            let out_value = OutputValue {
                ema,
                time: input_value.time,
            };

            let msg = (self.fn_output_msgbus)(&out_value);
            if let Some(msg) = msg {
                self.output_msgbus
                    .send(msg.to_message())
                    .await
                    .map_err(|_| Error::SendToMsgbus)?;
            }

            self.output
                .send(ValueTime {
                    value: out_value.ema,
                    time: out_value.time,
                })
                .await
                .map_err(|_| Error::AlgTaskUnexpectedEnd(String::from("EMA")))?;
        }

        let err = String::from("EMA");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}

struct EmaCalc {
    pub kind: EmaKind,
    pub prev_value: ValueTime,
    pub prev_ema: f64,
    pub value: ValueTime,
    pub time_window: Duration,
}
impl EmaCalc {
    pub fn calc(self) -> f64 {
        match self.kind {
            EmaKind::Last => self.last(),
            EmaKind::Next => self.next(),
            EmaKind::Linear => self.linear(),
        }
    }

    fn last(self) -> f64 {
        let w = (-(self.value.time - self.prev_value.time) / self.time_window).exp();
        self.prev_ema * w + self.prev_value.value * (1.0 - w)
    }

    fn next(self) -> f64 {
        let w = (-(self.value.time - self.prev_value.time) / self.time_window).exp();
        self.prev_ema * w + self.value.value * (1.0 - w)
    }

    fn linear(self) -> f64 {
        let tmp = (self.value.time - self.prev_value.time) / self.time_window;
        let w = (-tmp).exp();
        let w2 = (1.0 - w) / tmp;
        self.prev_ema * w + self.value.value * (1.0 - w2) + self.prev_value.value * (w2 - w)
    }
}
