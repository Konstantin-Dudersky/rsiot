use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Mutex;

use crate::{
    executor::{sleep, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::super::{
    plc::{FunctionBlockBase, IFunctionBlock},
    ConfigRetention,
};

pub struct ExportCurrentState<TMsg, TService, I, Q, S>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    pub in_out: CmpInOut<TMsg, TService>,
    pub config_retention: Option<ConfigRetention<TMsg, I, Q, S>>,
    pub fb_main: Arc<Mutex<FunctionBlockBase<I, Q, S>>>,
}

impl<TMsg, TService, I, Q, S> ExportCurrentState<TMsg, TService, I, Q, S>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    I: Clone + Default + Send + Serialize,
    Q: Clone + Default + Send + Serialize,
    S: Clone + Default + Send + Serialize,
    FunctionBlockBase<I, Q, S>: IFunctionBlock<I, Q, S>,
{
    pub async fn spawn(self) -> super::Result<()> {
        let Some(config_retention) = self.config_retention else {
            return Ok(());
        };
        loop {
            sleep(config_retention.save_period).await;
            let input;
            let output;
            let stat;
            {
                let fb_main = self.fb_main.lock().await;
                input = fb_main.input.clone();
                output = fb_main.output.clone();
                stat = fb_main.stat.clone();
            }
            let msgs = (config_retention.fn_export)(&input, &output, &stat);
            let Some(msgs) = msgs else { continue };
            for msg in msgs {
                self.in_out.send_output(msg).await.unwrap();
            }
        }
    }
}
