use std::time::Duration;

use embedded_svc::{
    http::{Headers, Method},
    io::{Read, Write},
};
use esp_idf_svc::http::server::{Configuration as HttpServerConfiguration, EspHttpServer};
use tokio::time::sleep;
use tracing::{info, trace, warn};

use crate::{
    executor::CmpInOut,
    message::{system_messages, Message, MsgData, MsgDataBound, ServiceBound},
};

use super::Config;

/// Заголовки для разрешения CORS
const HEADERS: [(&str, &str); 4] = [
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Max-Age", "600"),
    ("Access-Control-Allow-Methods", "PUT,POST,GET,OPTIONS"),
    ("Access-Control-Allow-Headers", "*"),
];

pub async fn fn_process<TMsg, TService>(
    mut in_out: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    while let Ok(msg) = in_out.recv_input().await {
        match msg.data {
            MsgData::System(system_messages::System::EspWifiConnected) => break,
            _ => continue,
        }
    }
    info!("Starting cmp_esp_http_server");

    let http_config = HttpServerConfiguration {
        http_port: config.port,
        ..Default::default()
    };

    let mut server = loop {
        info!("trying to create EspHttpServer");
        let server = EspHttpServer::new(&http_config);
        match server {
            Ok(server) => break server,
            Err(err) => {
                let err = format! {"Error EspHttpServer creation: {}", err};
                warn!("{}", err);
            }
        }
        sleep(Duration::from_secs(2)).await;
    };

    // Запрос чтения всех сообщений
    let cache_clone = in_out.cache.clone();
    server
        .fn_handler("/messages", Method::Get, move |request| {
            trace!("Get request, all messages");
            let mut msgs_json: Vec<String> = vec![];
            {
                let lock = cache_clone.blocking_read();
                for msg in lock.values() {
                    if !msg.is_route_enabled(&config.this_service, &config.client_service) {
                        continue;
                    }
                    let msg_json = msg.serialize().unwrap();
                    msgs_json.push(msg_json);
                }
            }
            let json = msgs_json.join(",");
            let json = format!("[{}]", json);
            let mut response = request.into_response(200, None, &HEADERS).unwrap();
            response.write_all(json.as_bytes()).unwrap();
            Ok(()) as super::Result<()>
        })
        .unwrap();

    // Запись одного сообщения
    let in_out_clone = in_out.clone();
    server
        .fn_handler("/messages", Method::Put, move |mut request| {
            trace!("Put request");

            let len = request.content_len().unwrap_or(0) as usize;
            let mut buf = vec![0; len];
            request.read_exact(&mut buf).unwrap();
            let buf_str = String::from_utf8_lossy(&buf);
            let mut response = request.into_response(200, None, &HEADERS).unwrap();
            let msg = Message::deserialize(&buf_str);
            match msg {
                Ok(val) => in_out_clone.send_output_blocking(val).unwrap(),
                Err(err) => {
                    let err = format!("{:?}", err);
                    response.write_all(err.as_bytes()).unwrap();
                }
            }

            Ok(()) as super::Result<()>
        })
        .unwrap();

    // Запись одного сообщения (копия PUT)
    let in_out_clone = in_out.clone();
    server
        .fn_handler("/messages", Method::Post, move |mut request| {
            trace!("Post request");

            let len = request.content_len().unwrap_or(0) as usize;
            let mut buf = vec![0; len];
            request.read_exact(&mut buf).unwrap();
            let buf_str = String::from_utf8_lossy(&buf);
            let mut response = request.into_response(200, None, &HEADERS).unwrap();
            let msg = Message::deserialize(&buf_str);
            match msg {
                Ok(val) => in_out_clone.send_output_blocking(val).unwrap(),
                Err(err) => {
                    let err = format!("{:?}", err);
                    response.write_all(err.as_bytes()).unwrap();
                }
            }

            Ok(()) as super::Result<()>
        })
        .unwrap();

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}
