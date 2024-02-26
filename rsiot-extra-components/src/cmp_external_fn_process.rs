use std::sync::Arc;

use async_trait::async_trait;
use futures::future::{BoxFuture, LocalBoxFuture};
use tokio::sync::Mutex;
use tracing::info;

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, ComponentResult, IComponentProcess,
};
use rsiot_messages_core::*;

#[cfg(feature = "single-thread")]
// #[derive(Clone, Debug)]
pub struct Config {
    pub fn_process: Box<dyn Fn() -> LocalBoxFuture<'static, ComponentResult>>,
}

// #[cfg(not(feature = "single-thread"))]
// // #[derive(Clone)]
// pub struct Config
// // where
// //     Box<dyn Fn() -> BoxFuture<'static, ComponentResult>>: Send,
// {
//     // pub fn_process: Box<dyn Fn() -> BoxFuture<'static, ComponentResult>>,
//     // pub fn_process: Arc<Mutex<Box<dyn Fn() -> BoxFuture<'static, ComponentResult>>>>,
//     pub fn_process: u16,
// }

#[cfg(not(feature = "single-thread"))]
struct Config
where
    Self: Send,
    Box<dyn Fn() -> ()>: Send,
{
    func: Box<dyn Fn() -> () + Send>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: Config,
        _input: CmpInput<TMsg>,
        _output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        info!("Start component cmp_extrenal_fn_process");
        // let fn_ = config.fn_process.lock().await;
        // (fn_)().await
        Ok(())
    }
}

#[cfg(feature = "single-thread")]
pub type Cmp<TMessage> = Component<Config, TMessage>;
