//! Компонент для подключения через websocket server.
//!
//! Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам
//!

use tokio::{
    net::TcpListener,
    spawn,
    sync::{broadcast, mpsc},
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_extra_components::{cmp_cache, cmpbase_mpsc_to_broadcast};
use rsiot_messages_core::IMessage;

use crate::{config::Config, errors::Errors};

use super::{
    async_task_utils::cancellable_task,
    handle_ws_connection::handle_ws_connection,
};

/// TODO - получение сообщений от клиентов и перенаправление в выходной поток
pub async fn process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    info!("Component component_websocket_server started");

    let cancel = CancellationToken::new();
    let (msgs_cache_output, msgs_broadcast_input) =
        mpsc::channel::<TMessage>(1000);
    let (msgs_broadcast_output, mut _rx_broadcast) =
        broadcast::channel::<TMessage>(1000);

    let cache = cmp_cache::create_cache::<TMessage>();

    // кэшируем данные
    let _task_cache = cmp_cache::new(cmp_cache::Config {
        cache: cache.clone(),
    })
    .set_and_spawn(input, Some(msgs_cache_output));

    // распространяем данные через broadcast
    let future = cmpbase_mpsc_to_broadcast::new(
        Some(msgs_broadcast_input),
        msgs_broadcast_output.clone(),
    );
    spawn(cancellable_task(future, cancel.clone()));

    loop {
        let result = task_main(
            cancel.clone(),
            msgs_broadcast_output.clone(),
            &output,
            cache.clone(),
            config.clone(),
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage>(
    cancel: CancellationToken,
    msgs_broadcast_output: broadcast::Sender<TMessage>,
    _msgs_output: &StreamOutput<TMessage>,
    cache: cmp_cache::CacheType<TMessage>,
    config: Config<TMessage>,
) -> Result<(), Errors>
where
    TMessage: IMessage + 'static,
{
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = create_tcp_listener(addr).await?;

    // слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok(stream_and_addr) = listener.accept().await {
        let future = handle_ws_connection(
            stream_and_addr,
            msgs_broadcast_output.subscribe(),
            cache.clone(),
            config.fn_send_to_client,
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
