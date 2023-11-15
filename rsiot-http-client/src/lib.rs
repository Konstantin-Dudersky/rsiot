mod error;
mod types;

use reqwest::{Client, Response, StatusCode};
use tokio::time::{sleep, Duration};
use tracing::error;
use url::Url;

use rsiot_http_client_config as hcc;
use rsiot_http_client_config::{
    HttpClientConfig, RequestCyclic, RequestParam, RequestParamKind,
};
use rsiot_messages_core::IMessage;

use crate::error::Error;
use crate::types::Result_;

pub async fn component_http_client<TMessage>(
    config: HttpClientConfig<TMessage>,
) -> ()
where
    TMessage: IMessage,
{
    loop {
        let url = config.connection_config.url.clone();
        for req in &config.requests_cyclic {
            process_request_and_response(&url, req).await.unwrap();
        }

        sleep(Duration::from_secs(2)).await;
    }
}

async fn process_request_and_response<TMessage>(
    url: &Url,
    req: &RequestCyclic<TMessage>,
) -> Result_<Vec<TMessage>>
where
    TMessage: IMessage,
{
    let response = send_request(url.clone(), &req.request_params).await;
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
            req.request_params, text
        );
        return Ok(msgs);
    }
    let msgs = (req.on_success)(text);
    return Ok(msgs);
}

/// Выполнение запроса
async fn send_request(url: Url, req: &RequestParam) -> Result_<Response> {
    let url = url.join(&req.endpoint).map_err(|err| {
        let err = err.to_string();
        Error::ConfigurationError(err)
    })?;
    let client = Client::new();
    let response = match req.kind {
        RequestParamKind::Get => client.get(url).send().await?,
        RequestParamKind::Put => todo!(),
        RequestParamKind::Post => todo!(),
    };
    Ok(response)
}
