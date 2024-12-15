use std::time::Duration;

use reqwest::{Client, StatusCode};
use tokio::time::sleep;
use tracing::{error, info, trace, warn};

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{MsgDataBound, ServiceBound},
};

use super::{
    config::{Config, LineProtocolItem},
    error::Error,
};

pub async fn fn_process<TMsg, TService>(
    in_out: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    info!("Starting influxdb client, configuration: {:?}", config);

    loop {
        let res = task_main(in_out.clone(), config.clone()).await;
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

async fn task_main<TMsg, TService>(
    mut input: CmpInOut<TMsg, TService>,
    config: Config<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    while let Ok(msg) = input.recv_input().await {
        let datapoints = (config.fn_input)(&msg);
        let datapoints = match datapoints {
            Some(datapoints) => datapoints,
            None => continue,
        };
        handle_request(datapoints, config.clone()).await?;
    }
    Err(super::Error::TaskEndInput)
}

async fn handle_request<TMsg>(
    datapoints: Vec<LineProtocolItem>,
    config: Config<TMsg>,
) -> super::Result<()>
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
        .map_err(super::Error::Config)?
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
