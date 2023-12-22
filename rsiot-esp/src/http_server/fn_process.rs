//! Схема потока сообщений:
//!
//! ```text
//!       ----> cache
//! input       plc ------> output
//!       ---------------->
//! ```

use std::time::{Duration, Instant};

use tokio::{spawn, sync::mpsc, time::sleep};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_extra_components::cmp_cache;
use rsiot_messages_core::IMessage;

use super::config::Config;

pub async fn fn_process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config,
) where
    TMessage: IMessage + 'static,
{
}
