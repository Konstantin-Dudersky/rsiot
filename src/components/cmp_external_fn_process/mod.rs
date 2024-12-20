//! Тестирование документации:
//!
//! ```bash
//! cargo test components::cmp_external_fn_process --features="executor" --target="x86_64-unknown-linux-gnu";
//! cargo test --doc components::cmp_external_fn_process --features="executor" --target="x86_64-unknown-linux-gnu";
//!
//! cargo test components::cmp_external_fn_process --features="executor, single-thread" --target="x86_64-unknown-linux-gnu";
//! cargo test --doc components::cmp_external_fn_process --features="executor, single-thread" --target="x86_64-unknown-linux-gnu";
//! ```

use async_trait::async_trait;

#[cfg(feature = "single-thread")]
pub use futures::future::LocalBoxFuture;

#[cfg(not(feature = "single-thread"))]
pub use futures::future::BoxFuture;

use crate::{
    executor::{CmpInOut, CmpResult, Component, ComponentError, IComponentProcess},
    message::*,
};

#[cfg(feature = "single-thread")]
type FnProcess<TMsg, TService> =
    Box<dyn Fn(CmpInOut<TMsg, TService>) -> LocalBoxFuture<'static, CmpResult>>;

#[cfg(not(feature = "single-thread"))]
type FnProcess<TMsg, TService> =
    Box<dyn Fn(CmpInOut<TMsg, TService>) -> BoxFuture<'static, CmpResult> + Send + Sync>;

/// Настройки cmp_external_fn_process
pub struct Config<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Внешняя функция для выполнения
    ///
    /// Выполняемую асинхронную функцию `fn_external` необходимо обернуть в функцию.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use futures::future::LocalBoxFuture;
    /// use tokio::time::sleep;
    /// use tracing::info;
    ///
    /// use rsiot::{
    ///     components::cmp_external_fn_process,
    ///     executor::{CmpInOut, ComponentResult},
    ///     message::{example_message::*, *},
    /// };
    ///
    /// fn fn_process_wrapper<TMsg>(
    ///     in_out: CmpInOut<TMsg>,
    /// ) -> LocalBoxFuture<'static, ComponentResult>
    /// where
    ///     TMsg: MsgDataBound + 'static,
    /// {
    ///     Box::pin(async { fn_process(in_out).await })
    /// }
    /// async fn fn_process<TMsg>(_in_out: CmpInOut<TMsg>) -> ComponentResult {
    ///     loop {
    ///         info!("External fn process");
    ///         sleep(Duration::from_secs(2)).await;
    ///     }
    /// }
    ///
    /// let _config = cmp_external_fn_process::Config {
    ///     fn_process: Box::new(fn_process_wrapper::<Custom>),
    /// };
    /// # // insert-end
    /// ```
    #[cfg(feature = "single-thread")]
    pub fn_process: FnProcess<TMsg, TService>,

    /// Внешняя функция для выполнения
    ///
    /// Выполняемую асинхронную функцию `fn_external` необходимо обернуть в функцию.
    ///
    /// # Пример
    ///
    /// ```rust
    /// # // insert-start test multi_thread
    /// use std::time::Duration;
    ///
    /// use futures::future::BoxFuture;
    /// use tokio::time::sleep;
    /// use tracing::info;
    ///
    /// use rsiot::{
    ///     components::cmp_external_fn_process,
    ///     executor::{CmpInOut, ComponentResult},
    ///     message::{example_message::*, *},
    /// };
    ///
    /// fn fn_process_wrapper<TMsg>(in_out: CmpInOut<TMsg>) -> BoxFuture<'static, ComponentResult>
    /// where
    ///     TMsg: MsgDataBound + 'static,
    /// {
    ///     Box::pin(async { fn_process(in_out).await })
    /// }
    ///
    /// async fn fn_process<TMsg>(_in_out: CmpInOut<TMsg>) -> ComponentResult {
    ///     loop {
    ///         info!("External fn process");
    ///         sleep(Duration::from_secs(2)).await;
    ///     }
    /// }
    ///
    /// let _config = cmp_external_fn_process::Config {
    ///     fn_process: Box::new(fn_process_wrapper::<Custom>),
    /// };
    /// # // insert-end
    /// ```
    #[cfg(not(feature = "single-thread"))]
    pub fn_process: FnProcess<TMsg, TService>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[async_trait(?Send)]
impl<TMsg, TService> IComponentProcess<Config<TMsg, TService>, TMsg, TService>
    for Component<Config<TMsg, TService>, TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    async fn process(
        &self,
        config: Config<TMsg, TService>,
        in_out: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        (config.fn_process)(
            in_out.clone_with_new_id("cmp_extrenal_fn_process", AuthPermissions::FullAccess),
        )
        .await
    }
}

/// Компонент cmp_external_fn_process
pub type Cmp<TMsg, TService> = Component<Config<TMsg, TService>, TMsg, TService>;

#[cfg(test)]
mod tests {

    #[cfg(feature = "single-thread")]
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn single_thread() {
        use std::time::Duration;

        use example_service::Service;
        use futures::future::LocalBoxFuture;

        #[cfg(target_arch = "wasm32")]
        use gloo::timers::future::sleep;
        #[cfg(not(target_arch = "wasm32"))]
        use tokio::time::sleep;

        use tracing::info;

        use crate::{
            components::cmp_external_fn_process,
            executor::{CmpInOut, CmpResult},
            message::{example_message::*, *},
        };

        fn fn_process_wrapper<TMsg, TService>(
            in_out: CmpInOut<TMsg, TService>,
        ) -> LocalBoxFuture<'static, CmpResult>
        where
            TMsg: MsgDataBound + 'static,
            TService: ServiceBound + 'static,
        {
            Box::pin(async { fn_process(in_out).await })
        }
        async fn fn_process<TMsg, TService>(_in_out: CmpInOut<TMsg, TService>) -> CmpResult
        where
            TMsg: MsgDataBound,
            TService: ServiceBound,
        {
            loop {
                info!("External fn process");
                sleep(Duration::from_secs(2)).await;
            }
        }

        let _config = cmp_external_fn_process::Config {
            fn_process: Box::new(fn_process_wrapper::<Custom, Service>),
        };
    }

    #[cfg(not(feature = "single-thread"))]
    #[test]
    fn multi_thread() {
        use std::time::Duration;

        use example_service::Service;
        use futures::future::BoxFuture;
        use tokio::time::sleep;
        use tracing::info;

        use crate::{
            components::cmp_external_fn_process,
            executor::{CmpInOut, CmpResult},
            message::{example_message::*, *},
        };

        fn fn_process_wrapper<TMsg, TService>(
            in_out: CmpInOut<TMsg, TService>,
        ) -> BoxFuture<'static, CmpResult>
        where
            TMsg: MsgDataBound + 'static,
            TService: ServiceBound + 'static,
        {
            Box::pin(async { fn_process(in_out).await })
        }

        async fn fn_process<TMsg, TService>(_in_out: CmpInOut<TMsg, TService>) -> CmpResult
        where
            TMsg: MsgDataBound + 'static,
            TService: ServiceBound + 'static,
        {
            loop {
                info!("External fn process");
                sleep(Duration::from_secs(2)).await;
            }
        }

        let _config = cmp_external_fn_process::Config {
            fn_process: Box::new(fn_process_wrapper::<Custom, Service>),
        };
    }
}
