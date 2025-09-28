//! Компонент для подключения через websocket server.
//!
//! Перенаправляет поток входящих сообщений подключенным вебсокет-клиентам
//!

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use futures::StreamExt;
use futures::TryFutureExt;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, broadcast, mpsc},
    task::JoinSet,
    time::{Duration, sleep},
};
use tokio_tungstenite::accept_async;
use tracing::{error, info, warn};

use crate::executor::MsgBusInput;
use crate::executor::MsgBusOutput;
use crate::serde_utils::SerdeAlg;
use crate::{components::shared_tasks, executor::join_set_spawn, message::MsgDataBound};

use super::{
    ServerToClientCache,
    config::{Config, WebsocketMessage},
    errors::Error,
    tasks,
};

pub async fn fn_process<TMsg, TServerToClient, TClientToServer>(
    input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
    config: Config<TMsg, TServerToClient, TClientToServer>,
) -> Result<(), Error>
where
    TMsg: MsgDataBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    info!(
        "Component cmp_websocket_server started. Config: {:?}",
        config
    );

    let result = task_main(input, output, config.clone()).await;
    match result {
        Ok(_) => (),
        Err(err) => error!("{:?}", err),
    }
    sleep(Duration::from_secs(2)).await;

    Err(Error::FnProcessEnd)
}

async fn task_main<TMsg, TServerToClient, TClientToServer>(
    input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
    config: Config<TMsg, TServerToClient, TClientToServer>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    let addr = format!("0.0.0.0:{}", config.port);

    let listener = create_tcp_listener(addr).await?;

    let cache = Arc::new(Mutex::new(HashMap::new()));

    let serde_alg = SerdeAlg::new(config.serde_alg);

    let (ch_tx_msgbus_to_mpsc, ch_rx_msgbus_to_mpsc) = mpsc::channel(1000);
    let (ch_tx_input_to_clients, ch_rx_input_to_clients) = broadcast::channel(1000);
    let (ch_tx_clients_to_output, ch_rx_clients_to_output) = mpsc::channel(1000);
    let (ch_tx_mpsc_to_msgbus, ch_rx_mpsc_to_msgbus) = mpsc::channel(1000);

    let mut task_set = JoinSet::new();

    // Пересылка входящих сообщений ----------------------------------------------------------------
    let task = shared_tasks::msgbus_to_mpsc_new::MsgBusToMpsc {
        input,
        output: ch_tx_msgbus_to_mpsc,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_websocket_server | msgbus_to_mpsc",
        task.spawn().map_err(super::Error::SharedTaskMsgBusToMpsc),
    );

    // Преобразование входящих сообщений -----------------------------------------------------------
    let task = tasks::Input {
        input: ch_rx_msgbus_to_mpsc,
        output: ch_tx_input_to_clients,
        fn_input: config.fn_server_to_client,
        cache: cache.clone(),
    };
    join_set_spawn(&mut task_set, "cmp_websocket_server | input", task.spawn());

    // Создание исходящих сообщений ----------------------------------------------------------------
    let task = tasks::Output {
        input: ch_rx_clients_to_output,
        output: ch_tx_mpsc_to_msgbus,
        fn_output: config.fn_client_to_server,
    };
    join_set_spawn(&mut task_set, "cmp_websocket_server | output", task.spawn());

    // Исходящие сообщения в шину сообщений --------------------------------------------------------
    let task = shared_tasks::mpsc_to_msgbus_new::MpscToMsgBus {
        input: ch_rx_mpsc_to_msgbus,
        output,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_websocket_server | mpsc_to_msgbus",
        task.spawn().map_err(super::Error::SharedTaskMpscToMsgBus),
    );

    // Слушаем порт, при получении запроса создаем новое подключение WS
    while let Ok(stream_and_addr) = listener.accept().await {
        let ch_rx_input_to_clients = ch_rx_input_to_clients.resubscribe();
        let ch_tx_clients_to_output = ch_tx_clients_to_output.clone();
        let task = handle_ws_connection(
            ch_rx_input_to_clients,
            ch_tx_clients_to_output,
            cache.clone(),
            stream_and_addr,
            serde_alg,
        );
        join_set_spawn(&mut task_set, "cmp_websocket_server | connection", task);
    }

    Ok(())
}

pub async fn handle_ws_connection<TServerToClient, TClientToServer>(
    input: broadcast::Receiver<TServerToClient>,
    output: mpsc::Sender<TClientToServer>,
    cache: ServerToClientCache<TServerToClient>,
    stream_and_addr: (TcpStream, SocketAddr),
    serde_alg: SerdeAlg,
) -> super::Result<()>
where
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    let addr = stream_and_addr.1;
    let result =
        _handle_ws_connection(input, output, cache.clone(), stream_and_addr, serde_alg).await;
    match result {
        Ok(_) => (),
        Err(err) => {
            warn!("Websocket client from address: {}, error: {}", addr, err)
        }
    }
    info!("Connection closed");

    Ok(())
}

async fn _handle_ws_connection<TServerToClient, TClientToServer>(
    input: broadcast::Receiver<TServerToClient>,
    output: mpsc::Sender<TClientToServer>,
    cache: ServerToClientCache<TServerToClient>,
    stream_and_addr: (TcpStream, SocketAddr),
    serde_alg: SerdeAlg,
) -> super::Result<()>
where
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    info!("Incoming TCP connection from: {}", stream_and_addr.1);
    let ws_stream = accept_async(stream_and_addr.0).await?;
    let (websocket_write, websocket_read) = ws_stream.split();
    info!("WebSocket connection established: {:?}", stream_and_addr.1);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Отправление данных клиенту
    let task = tasks::SendToClient {
        input: input.resubscribe(),
        websocket_write,
        cache: cache.clone(),
        serde_alg,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_websocket_server | send_to_client",
        task.spawn(),
    );

    // Получение данных от клиента
    let task = tasks::RcvFromClient {
        output: output.clone(),
        websocket_read,
        serde_alg,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_websocket_server | recv_from_client",
        task.spawn(),
    );

    while let Some(res) = task_set.join_next().await {
        let err = match res {
            Ok(val) => match val {
                Ok(_) => continue,
                Err(err) => format!("{err}"),
            },
            Err(err) => format!("{err}"),
        };
        warn!("Connection error: {}", err);
        task_set.shutdown().await;
    }
    Ok(())
}

async fn create_tcp_listener(addr: String) -> super::Result<TcpListener> {
    let listener = TcpListener::bind(&addr).await;
    let listener = listener.map_err(Error::BindToPort)?;
    info!("Listening on: {}", addr);
    Ok(listener)
}
