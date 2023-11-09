use tokio::{
    net::TcpListener,
    spawn,
    sync::{broadcast, mpsc},
};
use tokio_util::sync::CancellationToken;
use tracing::info;

use rsiot_channel_utils::{
    component_cache, component_mpsc_to_broadcast, create_cache,
};
use rsiot_messages_core::IMessage;

use crate::Errors;

use super::{
    async_task_utils::cancellable_task,
    handle_ws_connection::handle_ws_connection,
};

/// Компонент для подключения через websocket server.
pub async fn component_websocket_server<TMessage>(
    cancel: CancellationToken,
    msgs_input: mpsc::Receiver<TMessage>,
    _msgs_output: mpsc::Sender<TMessage>,
    ws_port: u16,
) -> Result<(), Errors>
where
    TMessage: IMessage + 'static,
{
    let addr = format!("0.0.0.0:{}", ws_port);

    let listener = create_tcp_listener(addr).await?;

    let (msgs_cache_output, msgs_broadcast_input) =
        mpsc::channel::<TMessage>(1000);
    let (msgs_broadcast_output, mut _rx_broadcast) =
        broadcast::channel::<TMessage>(1000);

    let cache = create_cache();

    // кэшируем данные
    let future = component_cache(msgs_input, msgs_cache_output, cache.clone());
    spawn(cancellable_task(future, cancel.clone()));

    // распространяем данные через broadcast
    let future = component_mpsc_to_broadcast(
        msgs_broadcast_input,
        msgs_broadcast_output.clone(),
    );
    spawn(cancellable_task(future, cancel.clone()));

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok((stream, addr)) = listener.accept().await {
        let future = handle_ws_connection(
            stream,
            addr,
            msgs_broadcast_output.subscribe(),
            cache.clone(),
        );
        spawn(cancellable_task(future, cancel.clone()));
    }

    Ok(())
}

async fn create_tcp_listener(addr: String) -> Result<TcpListener, Errors> {
    let listener = TcpListener::bind(&addr).await;
    let listener = match listener {
        Ok(value) => value,
        Err(error) => {
            return Err(Errors::BindToPortError(error));
        }
    };
    info!("Listening on: {}", addr);
    Ok(listener)
}
