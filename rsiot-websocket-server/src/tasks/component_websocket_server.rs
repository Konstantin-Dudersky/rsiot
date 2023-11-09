use tokio::{
    net::TcpListener,
    spawn,
    sync::{broadcast, mpsc},
};
use tokio_util::sync::CancellationToken;
use tracing::info;

use rsiot_channel_utils::component_mpsc_to_broadcast;
use rsiot_messages_core::IMessage;

use crate::{cancellable_task, Errors};

/// Компонент для подключения через websocket server.
pub async fn component_websocket_server<TMessage>(
    cancel: CancellationToken,
    input: mpsc::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    ws_port: u16,
) -> Result<(), Errors>
where
    TMessage: IMessage,
{
    let addr = format!("0.0.0.0:{}", ws_port);

    let listener = create_tcp_listener(addr).await?;

    let (tx_broadcast, mut _rx_broadcast) = broadcast::channel(128);

    // получаем данные из redis и рассылаем потокам websocket
    let future = component_mpsc_to_broadcast(input, tx_broadcast.clone());
    spawn(cancellable_task(future, cancel.clone()));

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok((stream, addr)) = listener.accept().await {
        let future =
            super::handle_ws_connection(stream, addr, tx_broadcast.subscribe());
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
