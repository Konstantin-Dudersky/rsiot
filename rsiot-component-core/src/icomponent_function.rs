//! Трейт для функции компонента
//!
//! https://stackoverflow.com/questions/58173711/how-can-i-store-an-async-function-in-a-struct-and-call-it-from-a-struct-instance
//!

use std::future::Future;

use futures::future::BoxFuture;

use crate::types::{CacheType, ComponentInput, ComponentOutput};

/// Трейт для функции компонента
pub trait IComponentFunction<TMessage, TConfig>: Send {
    fn call(
        &self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        config: TConfig,
        cache: CacheType<TMessage>,
    ) -> BoxFuture<'static, ()>;
}

impl<T, F, TMessage, TConfig> IComponentFunction<TMessage, TConfig> for T
where
    T: Fn(ComponentInput<TMessage>, ComponentOutput<TMessage>, TConfig, CacheType<TMessage>) -> F
        + Send,
    F: Future<Output = ()> + 'static + Send,
{
    fn call(
        &self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        config: TConfig,
        cache: CacheType<TMessage>,
    ) -> BoxFuture<'static, ()> {
        Box::pin(self(input, output, config, cache))
    }
}
