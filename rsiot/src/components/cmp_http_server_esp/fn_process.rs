use std::time::Duration;

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

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(in_out: CmpInOut<TMsg>, config: Config<TMsg>) -> super::Result<()>
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
            let mut response = request.into_ok_response().unwrap();
            response.write_all(json.as_bytes()).unwrap();
            Ok(()) as super::Result<()>
        })
        .unwrap();

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}
