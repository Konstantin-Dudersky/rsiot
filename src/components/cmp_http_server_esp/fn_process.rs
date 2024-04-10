use std::time::Duration;

use embedded_svc::{http::Headers, io::Read};
use esp_idf_svc::{
    http::{
        server::{Configuration as HttpServerConfiguration, EspHttpServer},
        Method,
    },
    io::Write,
};
use tokio::time::sleep;
use tracing::info;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::Config;

/// Заголовки для разрешения CORS
const HEADERS: [(&str, &str); 4] = [
    ("Access-Control-Allow-Origin", "*"),
    ("Access-Control-Max-Age", "600"),
    ("Access-Control-Allow-Methods", "PUT,POST,GET,OPTIONS"),
    ("Access-Control-Allow-Headers", "*"),
];

pub async fn fn_process<TMsg>(in_out: CmpInOut<TMsg>, _config: Config<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    sleep(Duration::from_secs(2)).await;

    let http_config = HttpServerConfiguration {
        ..Default::default()
    };

    let mut server = EspHttpServer::new(&http_config).unwrap();

    // Запрос чтения всех сообщений
    let cache_clone = in_out.cache.clone();
    server
        .fn_handler("/messages", Method::Get, move |request| {
            info!("Get request");
            let mut msgs_json: Vec<String> = vec![];
            {
                let lock = cache_clone.blocking_read();
                for msg in lock.values() {
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
            info!("Put request");

            let len = request.content_len().unwrap_or(0) as usize;
            let mut buf = vec![0; len];
            request.read_exact(&mut buf).unwrap();
            let buf_str = String::from_utf8_lossy(&buf);

            let request = request;

            let mut response = request.into_ok_response().unwrap();
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
            info!("Post request");

            let len = request.content_len().unwrap_or(0) as usize;
            let mut buf = vec![0; len];
            request.read_exact(&mut buf).unwrap();
            let buf_str = String::from_utf8_lossy(&buf);

            let mut response = request.into_ok_response().unwrap();
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
