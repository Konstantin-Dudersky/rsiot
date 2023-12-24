//! Для активации websocket-сервера в файле `sdkconfig.defaults` необходимо указать:
//! ```text
//! CONFIG_HTTPD_WS_SUPPORT=y
//! ````
//!
//! Пример реализации websoket -
//! https://docs.rs/crate/esp-idf-svc/latest/source/examples/ws_guessing_game.rs
//!
//! Пример реализации http -
//! https://docs.rs/crate/esp-idf-svc/latest/source/examples/json_post_handler.rs

use std::{
    thread::{self, sleep as std_sleep},
    time::Duration,
};

use embedded_svc::{http::Headers, io::Read};
use esp_idf_svc::{
    http::{
        server::{
            ws::EspHttpWsConnection, Configuration as HttpServerConfiguration, EspHttpServer,
        },
        Method,
    },
    io::Write,
    sys::EspError,
    ws::FrameType,
};
use tokio::time::sleep;

use rsiot_component_core::{Input, Output};
use rsiot_extra_components::cmp_cache::CacheType;
use rsiot_messages_core::IMessage;
use tracing::info;

use super::config::Config;

pub async fn fn_process<TMessage>(
    input: Input<TMessage>,
    output: Output<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    sleep(Duration::from_secs(2)).await;

    // Запускаем в синхронном треде, поскольку EspHttpServer не поддерживает Send
    let input_clone = input.resubscribe();
    let output_clone = output.clone();
    let cache_clone = config.cache.clone();
    let _thread = thread::spawn(|| create_server(input_clone, output_clone, cache_clone));

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}

fn create_server<TMessage>(
    input: Input<TMessage>,
    output: Output<TMessage>,
    cache: CacheType<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let http_config = HttpServerConfiguration {
        ..Default::default()
    };

    let mut server = EspHttpServer::new(&http_config).unwrap();

    // Запрос чтения всех сообщений
    let cache_clone = cache.clone();
    server
        .fn_handler("/messages", Method::Get, move |request| {
            let mut msgs_json: Vec<String> = vec![];
            {
                let lock = cache_clone.blocking_read();
                for msg in lock.values() {
                    let msg_json = msg.to_json()?;
                    msgs_json.push(msg_json);
                }
            }
            let json = msgs_json.join(",");
            let json = format!("[{}]", json);
            let mut response = request.into_ok_response()?;
            response.write_all(json.as_bytes())?;
            Ok(())
        })
        .unwrap();

    // Запись одного сообщения
    let output_clone = output.clone();
    server
        .fn_handler("/messages", Method::Put, move |mut request| {
            let len = request.content_len().unwrap_or(0) as usize;
            let mut buf = vec![0; len];
            request.read_exact(&mut buf)?;
            let buf_str = String::from_utf8_lossy(&buf);

            let mut response = request.into_ok_response()?;

            let msg = TMessage::from_json(&buf_str);
            match msg {
                Ok(val) => output_clone.blocking_send(val).unwrap(),
                Err(err) => {
                    let err = format!("{:?}", err);
                    response.write_all(err.as_bytes())?;
                }
            }

            Ok(())
        })
        .unwrap();

    // Отправка сообщений клиенту
    let cache_clone = cache.clone();
    server
        .ws_handler("/ws/get", move |ws| {
            match ws {
                EspHttpWsConnection::New(_, _) => info!("New WebSocket session ({})", ws.session()),
                EspHttpWsConnection::Receiving(_, _, _) => (),
                EspHttpWsConnection::Closed(_) => {
                    info!("Closed WebSocket session ({})", ws.session());
                    return Ok(());
                }
            }

            // Отправляем данные из кеша
            {
                let msgs = cache_clone.blocking_read();
                for msg in msgs.values() {
                    ws.send(FrameType::Text(false), msg.to_json().unwrap().as_bytes())?;
                }
            }

            // Отправляем входные сообщения
            while let Ok(msg) = input.resubscribe().blocking_recv() {
                ws.send(FrameType::Text(false), msg.to_json().unwrap().as_bytes())?
            }

            Ok::<(), EspError>(())
        })
        .unwrap();

    // Получение сообщений от клиента
    server
        .ws_handler("/ws/put", move |ws| {
            match ws {
                EspHttpWsConnection::New(_, _) => info!("New WebSocket session ({})", ws.session()),
                EspHttpWsConnection::Receiving(_, _, frame) => {
                    info!("Frame: {frame:?}");
                    ()
                }
                EspHttpWsConnection::Closed(_) => {
                    info!("Closed WebSocket session ({})", ws.session());
                    return Ok(());
                }
            }

            Ok::<(), EspError>(())
        })
        .unwrap();

    loop {
        std_sleep(Duration::from_secs(1));
    }
}
