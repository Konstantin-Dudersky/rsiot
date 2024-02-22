use std::time::Duration;

use reqwest::{Client, StatusCode};
use tokio::time::sleep;
use tracing::{error, info, trace, warn};

use rsiot_component_core::{CmpInput, CmpOutput, ComponentError};
use rsiot_messages_core::message_v2::MsgDataBound;

use crate::{
    config::{Config, LineProtocolItem},
    error::Error,
};

pub async fn fn_process<TMsg>(
    input: CmpInput<TMsg>,
    _output: CmpOutput<TMsg>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
{
    info!("Starting influxdb client, configuration: {:?}", config);

    loop {
        let res = task_main::<TMsg>(input.clone(), config.clone()).await;
        match res {
            Ok(_) => (),
            Err(err) => {
                error!("Error in influxdb-client: {:?}", err);
            }
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMsg>(mut input: CmpInput<TMsg>, config: Config<TMsg>) -> crate::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        let datapoints = (config.fn_input)(&msg);
        let datapoints = match datapoints {
            Some(datapoints) => datapoints,
            None => continue,
        };
        handle_request(datapoints, config.clone()).await?;
    }
    Ok(())
}

async fn handle_request<TMsg>(
    datapoints: Vec<LineProtocolItem>,
    config: Config<TMsg>,
) -> crate::Result<()>
where
    TMsg: MsgDataBound,
{
    trace!("New request to InfluxDB");
    let url = format!(
        "http://{host}:{port}/api/v2/write",
        host = config.host,
        port = config.port,
    );

    let lines = datapoints
        .iter()
        .map(String::try_from)
        .collect::<std::result::Result<Vec<String>, _>>()
        .map_err(crate::Error::Config)?
        .join("\n");

    let client = Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Token {}", config.token))
        .header("Accept", "application/json")
        .header("Content-Type", "text/plain; charset=utf-8")
        .query(&[
            ("org", config.org),
            ("bucket", config.bucket),
            ("precision", "ns".to_string()),
        ])
        .body(lines)
        .send()
        .await?;

    let status = response.status();
    if status == StatusCode::NO_CONTENT {
        return Ok(());
    }
    warn!("{status}");
    let text = response.text().await?;
    Err(Error::RequestParameters {
        status,
        message: text,
    })
}
