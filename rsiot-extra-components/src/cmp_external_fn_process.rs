//! Тестирование документации:
//!
//! ```bash
//! cargo test -p rsiot-extra-components --doc cmp_external_fn_process; cargo test -p rsiot-extra-components --doc cmp_external_fn_process --features single-thread
//! ```

use async_trait::async_trait;

#[cfg(feature = "single-thread")]
use futures::future::LocalBoxFuture;

#[cfg(not(feature = "single-thread"))]
use futures::future::BoxFuture;

use tracing::info;

use rsiot_component_core::{
    Cache, CmpInput, CmpOutput, Component, ComponentError, ComponentResult, IComponentProcess,
};
use rsiot_messages_core::*;

#[cfg(feature = "single-thread")]
type FnProcess<TMsg> = Box<
    dyn Fn(
        CmpInput<TMsg>,
        CmpOutput<TMsg>,
        Cache<TMsg>,
    ) -> LocalBoxFuture<'static, ComponentResult>,
>;

#[cfg(not(feature = "single-thread"))]
type FnProcess<TMsg> = Box<
    dyn Fn(CmpInput<TMsg>, CmpOutput<TMsg>, Cache<TMsg>) -> BoxFuture<'static, ComponentResult>
        + Send
        + Sync,
>;

pub struct Config<TMsg> {
    /// Внешняя функция для выполнения
    ///
    /// Выполняемую асинхронную функцию `fn_external` необходимо обернуть в функцию.
    ///
    /// ```rust
    /// # use rsiot_extra_components::cmp_external_fn_process;
    /// # // insert-start test single_thread
    /// use std::time::Duration;
    ///
    /// use futures::future::LocalBoxFuture;
    /// use tokio::time::sleep;
    /// use tracing::info;
    ///
    /// use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentResult};
    /// use rsiot_messages_core::{example_message::*, *};
    ///
    /// fn fn_process_wrapper<TMsg>(
    ///     input: CmpInput<TMsg>,
    ///     output: CmpOutput<TMsg>,
    ///     cache: Cache<TMsg>,
    /// ) -> LocalBoxFuture<'static, ComponentResult>
    /// where
    ///     TMsg: MsgDataBound + 'static,
    /// {
    ///     Box::pin(async { fn_process(input, output, cache).await })
    /// }
    /// async fn fn_process<TMsg>(
    ///     _input: CmpInput<TMsg>,
    ///     _output: CmpOutput<TMsg>,
    ///     _cache: Cache<TMsg>,
    /// ) -> ComponentResult {
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
    pub fn_process: FnProcess<TMsg>,

    /// Внешняя функция для выполнения
    ///
    /// Выполняемую асинхронную функцию `fn_external` необходимо обернуть в функцию.
    ///
    /// ```rust
    /// # use rsiot_extra_components::cmp_external_fn_process;
    /// # // insert-start test multi_thread
    /// use std::time::Duration;
    ///
    /// use futures::future::BoxFuture;
    /// use tokio::time::sleep;
    /// use tracing::info;
    ///
    /// use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentResult};
    /// use rsiot_messages_core::{example_message::*, *};
    ///
    /// fn fn_process_wrapper<TMsg>(
    ///     input: CmpInput<TMsg>,
    ///     output: CmpOutput<TMsg>,
    ///     cache: Cache<TMsg>,
    /// ) -> BoxFuture<'static, ComponentResult>
    /// where
    ///     TMsg: MsgDataBound + 'static,
    /// {
    ///     Box::pin(async { fn_process(input, output, cache).await })
    /// }
    ///
    /// async fn fn_process<TMsg>(
    ///     _input: CmpInput<TMsg>,
    ///     _output: CmpOutput<TMsg>,
    ///     _cache: Cache<TMsg>,
    /// ) -> ComponentResult {
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
    pub fn_process: FnProcess<TMsg>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[async_trait(?Send)]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        input: CmpInput<TMsg>,
        output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        info!("Start component cmp_extrenal_fn_process");
        (config.fn_process)(input, output, cache).await
    }
}

pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;

#[cfg(test)]
mod tests {

    use super::super::cmp_external_fn_process;

    #[cfg(feature = "single-thread")]
    #[test]
    fn single_thread() {
        use std::time::Duration;

        use futures::future::LocalBoxFuture;
        use tokio::time::sleep;
        use tracing::info;

        use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentResult};
        use rsiot_messages_core::{example_message::*, *};

        fn fn_process_wrapper<TMsg>(
            input: CmpInput<TMsg>,
            output: CmpOutput<TMsg>,
            cache: Cache<TMsg>,
        ) -> LocalBoxFuture<'static, ComponentResult>
        where
            TMsg: MsgDataBound + 'static,
        {
            Box::pin(async { fn_process(input, output, cache).await })
        }
        async fn fn_process<TMsg>(
            _input: CmpInput<TMsg>,
            _output: CmpOutput<TMsg>,
            _cache: Cache<TMsg>,
        ) -> ComponentResult {
            loop {
                info!("External fn process");
                sleep(Duration::from_secs(2)).await;
            }
        }

        let _config = cmp_external_fn_process::Config {
            fn_process: Box::new(fn_process_wrapper::<Custom>),
        };
    }

    #[cfg(not(feature = "single-thread"))]
    #[test]
    fn multi_thread() {
        use std::time::Duration;

        use futures::future::BoxFuture;
        use tokio::time::sleep;
        use tracing::info;

        use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentResult};
        use rsiot_messages_core::{example_message::*, *};

        fn fn_process_wrapper<TMsg>(
            input: CmpInput<TMsg>,
            output: CmpOutput<TMsg>,
            cache: Cache<TMsg>,
        ) -> BoxFuture<'static, ComponentResult>
        where
            TMsg: MsgDataBound + 'static,
        {
            Box::pin(async { fn_process(input, output, cache).await })
        }

        async fn fn_process<TMsg>(
            _input: CmpInput<TMsg>,
            _output: CmpOutput<TMsg>,
            _cache: Cache<TMsg>,
        ) -> ComponentResult {
            loop {
                info!("External fn process");
                sleep(Duration::from_secs(2)).await;
            }
        }

        let _config = cmp_external_fn_process::Config {
            fn_process: Box::new(fn_process_wrapper::<Custom>),
        };
    }
}
