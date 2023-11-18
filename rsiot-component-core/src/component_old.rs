use std::{future::Future, pin::Pin};

use tokio::{spawn, sync::mpsc, task::JoinHandle};

use crate::types::{StreamInput, StreamOutput};

pub type ProcessFunction<TMessage> = Box<
    dyn FnOnce(
        Option<StreamInput<TMessage>>,
        Option<StreamOutput<TMessage>>,
    ) -> Pin<Box<dyn Future<Output = ()>>>,
>;

pub fn force_boxed<TFuture, TMessage>(
    f: fn(
        Option<StreamInput<TMessage>>,
        Option<StreamOutput<TMessage>>,
    ) -> TFuture,
) -> ProcessFunction<TMessage>
where
    TFuture: Future<Output = ()> + 'static,
    TMessage: 'static,
{
    Box::new(move |n, m| Box::pin(f(n, m)))
}

//------------------------------------------------------------------------------

fn force_boxed2<T, TMessage>(
    f: fn(Option<StreamInput<TMessage>>, Option<StreamOutput<TMessage>>) -> T,
) -> impl Incrementer<TMessage>
where
    T: Future<Output = ()> + 'static,
{
    move |n, m| Box::pin(f(n, m)) as _
}

trait Incrementer<TMessage>:
    FnOnce(
    Option<StreamInput<TMessage>>,
    Option<StreamOutput<TMessage>>,
) -> Pin<Box<dyn Future<Output = ()>>>
{
}

impl<T, TMessage> Incrementer<TMessage> for T where
    T: FnOnce(
        Option<StreamInput<TMessage>>,
        Option<StreamOutput<TMessage>>,
    ) -> Pin<Box<dyn Future<Output = ()>>>
{
}

//------------------------------------------------------------------------------

pub struct Component<
    TMessage,
    TConfig,
    Fut: Future<Output = ()> + Send + 'static,
> {
    pub stream_input: Option<StreamInput<TMessage>>,
    pub stream_output: Option<StreamOutput<TMessage>>,
    pub config: TConfig,
    pub func: Box<dyn Fn(Option<String>) -> Fut>,
}

impl<TMessage, TConfig, TFunction> Component<TMessage, TConfig, TFunction>
where
    TFunction: Future<Output = ()> + Send + 'static,
{
    pub fn new(
        config: TConfig,
        func: impl Fn(Option<String>) -> TFunction + 'static,
    ) -> Self {
        Self {
            stream_input: None,
            stream_output: None,
            config,
            func: Box::new(func),
        }
    }

    pub fn spawn(&mut self) -> JoinHandle<()> {
        spawn((self.func)(Some("df".to_string())))
    }

    pub fn set_stream_input(&mut self, stream_input: StreamInput<TMessage>) {
        self.stream_input = Some(stream_input);
    }

    pub fn set_stream_output(&mut self, stream_output: StreamOutput<TMessage>) {
        self.stream_output = Some(stream_output);
    }
}
