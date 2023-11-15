use reqwest::{Client, Response, StatusCode};
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::error;
use url::Url;

use rsiot_http_client_config as hcc;
use rsiot_http_client_config::{RequestParam, RequestPeriodic};
use rsiot_messages_core::IMessage;

use crate::{error::Error, periodic_runner::PeriodicRunner, types::Result_};

pub async fn component_http_client<TMessage>(
    stream_input: Option<mpsc::Receiver<TMessage>>,
    stream_output: Option<mpsc::Sender<TMessage>>,
    config: hcc::HttpClientConfig<TMessage>,
) -> ()
where
    TMessage: IMessage,
{
    let mut periodic: Vec<PeriodicRunner> = config
        .requests_periodic
        .iter()
        .map(|r| PeriodicRunner::new(r.period.clone()))
        .collect();
    let url = config.connection_config.url.clone();

    loop {
        let mut msgs_output: Vec<TMessage> = vec![];

        for (idx, period) in periodic.iter_mut().enumerate() {
            if period.check() {
                let req = &config.requests_periodic[idx];
                let msgs =
                    process_request_and_response(&url, req).await.unwrap();
                msgs_output.extend(msgs);
            }
        }
        match &stream_output {
            Some(stream) => {
                for msg in msgs_output {
                    stream.send(msg).await.unwrap();
                }
            }
            None => (),
        }

        sleep(Duration::from_millis(10)).await;
    }
}

async fn process_request_and_response<TMessage>(
    url: &Url,
    req: &RequestPeriodic<TMessage>,
) -> Result_<Vec<TMessage>>
where
    TMessage: IMessage,
{
    let response = send_request(url.clone(), &req.request_param).await;
    let response = match response {
        Ok(val) => val,
        Err(err) => match err {
            Error::ReqwestError(err) => {
                error!("{:?}", err);
                let msgs = (req.on_failure)();
                return Ok(msgs);
            }
            _ => return Err(err),
        },
    };
    let status = response.status();
    let text = response.text().await?;
    if status != StatusCode::OK {
        let msgs = (req.on_failure)();
        error!(
            "Error on request.\nRequest params: {:?}\nResponse text: {:?}",
            req.request_param, text
        );
        return Ok(msgs);
    }
    let msgs = (req.on_success)(text);
    return Ok(msgs);
}

/// Выполнение запроса
async fn send_request(url: Url, req: &RequestParam) -> Result_<Response> {
    let endpoint = match req {
        RequestParam::Get(val) => val,
        RequestParam::Put(_) => todo!(),
        RequestParam::Post(_) => todo!(),
    };
    let url = url.join(endpoint).map_err(|err| {
        let err = err.to_string();
        Error::ConfigurationError(err)
    })?;
    let client = Client::new();
    let response = match req {
        RequestParam::Get(_) => client.get(url).send().await?,
        RequestParam::Put(_) => todo!(),
        RequestParam::Post(_) => todo!(),
    };
    Ok(response)
}
