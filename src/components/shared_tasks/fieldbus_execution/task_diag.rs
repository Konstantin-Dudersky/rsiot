use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    components_config::master_device::{
        FieldbusDiagMsg, FieldbusRequestWithIndex, RequestResponseBound,
    },
    executor::{Instant, MsgBusOutput, sleep},
    message::MsgDataBound,
};

use super::diag::{FieldbusDiag, StoreFieldbusDiag};

const LAST_ERRORS_LEN: usize = 10;

pub struct Diag<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<FieldbusDiagMsg>,
    pub output: MsgBusOutput<TMsg>,
    pub fn_diag: fn(&FieldbusDiag) -> TMsg,
    pub period: Duration,
    pub error_tokiompscsend: fn() -> TError,
}

impl<TMsg, TError> Diag<TMsg, TError>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        let mut sfd = StoreFieldbusDiag::new();

        let mut last_output = Instant::now();

        while let Some(fdm) = self.input.recv().await {
            match fdm {
                FieldbusDiagMsg::FieldbusRequestOk { duration } => {
                    sfd.msg_fieldbus_request_ok(duration)
                }
                FieldbusDiagMsg::FieldbusRequestErr { duration } => {
                    sfd.msg_fieldbus_request_err(duration)
                }
                FieldbusDiagMsg::DeviceInitCompleted { id, duration } => {
                    sfd.msg_device_init_completed(&id, duration)
                }
                FieldbusDiagMsg::DeviceRequestOk { id, duration } => {
                    sfd.msg_device_request_ok(&id, duration)
                }
                FieldbusDiagMsg::DeviceRequestErr {
                    id,
                    duration,
                    error,
                } => sfd.msg_device_request_err(&id, duration, error),
            }

            // Отправляем диагностическое сообщение
            if last_output.elapsed() >= self.period {
                last_output = Instant::now();
                let fd: FieldbusDiag = (&sfd).into();
                let msg = (self.fn_diag)(&fd);
                let msg = msg.to_message();

                self.output
                    .send(msg)
                    .await
                    .map_err(|_| (self.error_tokiompscsend)())?;
            }
        }

        Ok(())
    }
}
