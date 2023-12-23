use std::{thread::sleep as std_sleep, time::Duration};

use esp_idf_svc::{
    http::{
        server::{Configuration as HttpServerConfiguration, EspHttpServer},
        Method,
    },
    io::Write,
};
use tokio::time::sleep;

use rsiot_component_core::{Input, Output};
use rsiot_extra_components::cmp_cache::CacheType;
use rsiot_messages_core::IMessage;

use super::config::Config;

pub async fn fn_process<TMessage>(
    _input: Input<TMessage>,
    output: Output<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    // Необходимо подождать, пока поднимется Wi-Fi
    sleep(Duration::from_secs(2)).await;

    let output_clone = output.clone();
    let cache = config.cache.clone();

    // Запускаем в синхронном треде, поскольку EspHttpServer не поддерживает Send
    let _thread = std::thread::spawn(|| create_server(output_clone, cache));

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}

fn create_server<TMessage>(_output: Output<TMessage>, cache: CacheType<TMessage>)
where
    TMessage: IMessage + 'static,
{
    let mut server = EspHttpServer::new(&HttpServerConfiguration::default()).unwrap();
    server
        .fn_handler("/temperature", Method::Get, move |request| {
            let msg;
            {
                let lock = cache.blocking_lock();
                msg = lock.get("SetLedColor").map(|m| m.to_owned()).unwrap();
            }
            let mut response = request.into_ok_response()?;
            response.write_all(msg.to_json().unwrap().as_bytes())?;
            Ok(())
        })
        .unwrap();

    loop {
        std_sleep(Duration::from_secs(1));
    }
}
