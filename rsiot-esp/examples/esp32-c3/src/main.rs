use std::time::Duration;

use esp_idf_svc::{
    http::{
        server::{Configuration as HttpServerConfiguration, EspHttpServer},
        Method,
    },
    io::Write,
    {log::EspLogger, sys::link_patches},
};
use tokio::{main, spawn, sync::mpsc, time::sleep};

use rsiot::{
    cmp_plc,
    component::{
        cmp_add_input_stream, cmp_add_output_stream, cmp_delay, cmp_logger, ComponentChain,
    },
    message::msg_types::Value,
};
use rsiot_esp::cmp_http_server_esp;

use message::Message;
use rgb::RGB8;
use tracing::Level;

mod fb_main;
mod hal;
mod message;
mod ws2812rmt;

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let (input_hal_tx, input_hal_rx) = mpsc::channel(10);
    let (output_hal_tx, output_hal_rx) = mpsc::channel(10);

    spawn(hal::hal(Some(input_hal_rx), Some(output_hal_tx)));

    input_hal_tx
        .send(Message::SetLedColor(Value::new(RGB8::new(128, 128, 128))))
        .await
        .unwrap();

    let output_hal_config = cmp_add_input_stream::Config {
        channel: output_hal_rx,
    };

    let input_hal_config = cmp_add_output_stream::Config {
        channel: input_hal_tx,
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb_main::I, msg: &Message| match msg {
            Message::Button(_) => (),
            Message::SetLedColor(_) => (),
        },
        fn_output: |output: &fb_main::Q| {
            let msg1 = Message::SetLedColor(Value::new(output.color));
            vec![msg1]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
        buffer_size: 10,
    };

    let delay_config = cmp_delay::Config {
        delay: Duration::from_secs(2),
    };

    // настраиваем http server
    sleep(Duration::from_secs(5)).await;
    let mut server = EspHttpServer::new(&HttpServerConfiguration::default()).unwrap();
    server
        .fn_handler("/temperature", Method::Get, move |request| {
            let html = temperature(12.3);
            let mut response = request.into_ok_response()?;
            response.write_all(html.as_bytes())?;
            Ok(())
        })
        .unwrap();

    let mut chain = ComponentChain::<Message>::new(10)
        .add_cmp(cmp_add_input_stream::new(output_hal_config))
        .add_cmp(cmp_plc::new(plc_config))
        .add_cmp(cmp_add_output_stream::new(input_hal_config))
        .add_cmp(cmp_delay::new(delay_config))
        .add_cmp(cmp_logger::new(logger_config))
        .add_cmp(cmp_http_server_esp::new(cmp_http_server_esp::Config {}));

    chain.spawn().await;
}

fn temperature(val: f32) -> String {
    templated(format!("Chip temperature: {:.2}°C", val))
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}
