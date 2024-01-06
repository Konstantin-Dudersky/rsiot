//! Трейт для функции компонента
//!
//! https://stackoverflow.com/questions/58173711/how-can-i-store-an-async-function-in-a-struct-and-call-it-from-a-struct-instance
//!

use std::future::Future;

use futures::future::BoxFuture;

use crate::{
    error::ComponentError,
    types::{ComponentInput, ComponentOutput},
    Cache,
};

/// Трейт для функции компонента
pub trait IComponentFunction<TMessage, TConfig> {
    fn call(
        &self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        config: TConfig,
        cache: Cache<TMessage>,
    ) -> BoxFuture<'static, Result<(), ComponentError>>;
}

impl<T, F, TMessage, TConfig> IComponentFunction<TMessage, TConfig> for T
where
    T: Fn(ComponentInput<TMessage>, ComponentOutput<TMessage>, TConfig, Cache<TMessage>) -> F,
    F: Future<Output = Result<(), ComponentError>> + 'static,
{
    fn call(
        &self,
        input: ComponentInput<TMessage>,
        output: ComponentOutput<TMessage>,
        config: TConfig,
        cache: Cache<TMessage>,
    ) -> BoxFuture<'static, Result<(), ComponentError>> {
        Box::pin(self(input, output, config, cache))
    }
}
