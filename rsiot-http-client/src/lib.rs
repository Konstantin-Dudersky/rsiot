mod error;
mod types;

use reqwest::Client;
use tokio::time::{sleep, Duration};
use url::Url;

use rsiot_http_client_config::{HttpClientConfig, Request, RequestKind};
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
        let resp = send_request(
            config.connection_config.url.clone(),
            &config.requests_cyclic[0].request_params,
        )
        .await;
        println!("{:?}", resp);
        sleep(Duration::from_secs(2)).await;
    }
}

/// Выполнение запроса
async fn send_request(url: Url, req: &Request) -> Result_<String> {
    let url = url.join(&req.endpoint).map_err(|err| {
        let err = err.to_string();
        Error::ConfigurationError(err)
    })?;
    let client = Client::new();
    let resp = match req.kind {
        RequestKind::Get => {
            client.get(url).send().await.unwrap().text().await.unwrap()
        }
        RequestKind::Put => todo!(),
        RequestKind::Post => todo!(),
    };
    Ok(resp)
}
