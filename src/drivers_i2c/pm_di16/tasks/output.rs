use std::{sync::Arc, time::Duration};

use pm_firmware_lib::pm_di16_dc24sink_v0_0_2::{I2cRequest, I2cResponse};
use tokio::{
    sync::{mpsc::Sender, Mutex},
    time::sleep,
};
use tracing::warn;

use crate::{
    drivers_i2c::{I2cSlaveAddress, RsiotI2cDriverBase},
    message::{Message, MsgDataBound},
    serde_utils::postcard_serde,
};

use super::super::config::FnOutput;

pub struct Output<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    pub output: Sender<Message<TMsg>>,
    pub address: I2cSlaveAddress,
    pub fn_output_a_0: FnOutput<TMsg>,
    pub fn_output_a_1: FnOutput<TMsg>,
    pub fn_output_a_2: FnOutput<TMsg>,
    pub fn_output_a_3: FnOutput<TMsg>,
    pub fn_output_a_4: FnOutput<TMsg>,
    pub fn_output_a_5: FnOutput<TMsg>,
    pub fn_output_a_6: FnOutput<TMsg>,
    pub fn_output_a_7: FnOutput<TMsg>,

    pub fn_output_b_0: FnOutput<TMsg>,
    pub fn_output_b_1: FnOutput<TMsg>,
    pub fn_output_b_2: FnOutput<TMsg>,
    pub fn_output_b_3: FnOutput<TMsg>,
    pub fn_output_b_4: FnOutput<TMsg>,
    pub fn_output_b_5: FnOutput<TMsg>,
    pub fn_output_b_6: FnOutput<TMsg>,
    pub fn_output_b_7: FnOutput<TMsg>,
    pub fn_output_period: Duration,
    pub driver: Arc<Mutex<TDriver>>,
}

impl<TMsg, TDriver> Output<TMsg, TDriver>
where
    TMsg: MsgDataBound,
    TDriver: RsiotI2cDriverBase,
{
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            sleep(self.fn_output_period).await;
            let result = self.request().await;
            if let Err(err) = result {
                warn!("I2c request error: {err}");
            }
        }
    }

    async fn request(&self) -> super::Result<()> {
        let req = I2cRequest::GetInput;
        let req = postcard_serde::serialize_crc_deprecated(&req)?;

        let mut response = {
            let mut driver = self.driver.lock().await;
            driver
                .write_read(
                    self.address,
                    &req,
                    postcard_serde::MESSAGE_LEN,
                    Duration::from_millis(100),
                )
                .await
                .map_err(super::Error::I2c)?
        };

        let response: I2cResponse = postcard_serde::deserialize(&mut response)?;

        match response {
            I2cResponse::InputsState {
                a0,
                a1,
                a2,
                a3,
                a4,
                a5,
                a6,
                a7,
                b0,
                b1,
                b2,
                b3,
                b4,
                b5,
                b6,
                b7,
            } => {
                let mut msgs = vec![];
                msgs.push((self.fn_output_a_0)(a0));
                msgs.push((self.fn_output_a_1)(a1));
                msgs.push((self.fn_output_a_2)(a2));
                msgs.push((self.fn_output_a_3)(a3));
                msgs.push((self.fn_output_a_4)(a4));
                msgs.push((self.fn_output_a_5)(a5));
                msgs.push((self.fn_output_a_6)(a6));
                msgs.push((self.fn_output_a_7)(a7));
                msgs.push((self.fn_output_b_0)(b0));
                msgs.push((self.fn_output_b_1)(b1));
                msgs.push((self.fn_output_b_2)(b2));
                msgs.push((self.fn_output_b_3)(b3));
                msgs.push((self.fn_output_b_4)(b4));
                msgs.push((self.fn_output_b_5)(b5));
                msgs.push((self.fn_output_b_6)(b6));
                msgs.push((self.fn_output_b_7)(b7));

                for msg in msgs {
                    let Some(msg) = msg else { continue };
                    self.output
                        .send(msg)
                        .await
                        .map_err(|e| super::Error::TokioSyncMpscSender(e.to_string()))?;
                }
            }
        }

        Ok(())
    }
}
