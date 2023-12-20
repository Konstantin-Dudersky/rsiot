use std::{net::SocketAddr, sync::Arc, time::Instant};

use tokio::{
    spawn,
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
    time::{sleep, Duration},
};
use tracing::{debug, error, info, trace, warn};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

use crate::config::Config;

pub async fn fn_process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
}
