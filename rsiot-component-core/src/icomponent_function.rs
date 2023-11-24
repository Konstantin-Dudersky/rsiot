//! Трейт для функции компонента
//!
//! https://stackoverflow.com/questions/58173711/how-can-i-store-an-async-function-in-a-struct-and-call-it-from-a-struct-instance
//!

use std::future::Future;

use futures::future::BoxFuture;

use crate::types::{StreamInput, StreamOutput};

/// Трейт для функции компонента
pub trait IComponentFunction<TMessage, TConfig>: Send {
    fn call(
        &self,
        stream_input: StreamInput<TMessage>,
        stream_output: StreamOutput<TMessage>,
        config: TConfig,
    ) -> BoxFuture<'static, ()>;
}

impl<T, F, TMessage, TConfig> IComponentFunction<TMessage, TConfig> for T
where
    T: Fn(StreamInput<TMessage>, StreamOutput<TMessage>, TConfig) -> F + Send,
    F: Future<Output = ()> + 'static + Send,
{
    fn call(
        &self,
        stream_input: StreamInput<TMessage>,
        stream_output: StreamOutput<TMessage>,
        config: TConfig,
    ) -> BoxFuture<'static, ()> {
        Box::pin(self(stream_input, stream_output, config))
    }
}
