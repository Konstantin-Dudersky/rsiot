use std::time::Duration;

use embedded_svc::http::{client::Client as EspHttpClient, Method};
use esp_idf_svc::http::client::EspHttpConnection;
use tokio::{sync::mpsc, task::JoinSet};
use tracing::error;
use url::Url;

use crate::{
    components_config::http_client::{MsgRequest, MsgResponse},
    executor::join_set_spawn,
};

use super::{Error, Result};

pub struct HttpClient {
    pub input: mpsc::Receiver<MsgRequest>,
    pub output: mpsc::Sender<MsgResponse>,
    pub base_url: String,
    pub timeout: Duration,
}

impl HttpClient {
    pub async fn spawn(mut self) -> Result<()> {
        // Парсим url
        let base_url = Url::parse(&self.base_url).map_err(|err| {
            let err = err.to_string();
            let err = format!("Cannot parse url: {}", err);
            Error::TaskEndHttpClient(err)
        })?;

        // Создаем клиента
        let mut client = EspHttpClient::wrap(EspHttpConnection::new(&Default::default()).unwrap());

        let headers = [("accept", "text/plain")];

        let mut task_set = JoinSet::new();

        while let Some(request) = self.input.recv().await {}

        Err(Error::TaskEndHttpClient("".into()))
    }
}

pub struct SingleRequest {
    pub output: mpsc::Sender<MsgResponse>,
    pub base_url: Url,
    pub request: MsgRequest,
    client: Client,
}

impl SingleRequest {
    pub async fn spawn(self) -> super::Result<()> {
        let endpoint = self.request.get_endpoint().to_string();
        let full_url = self.base_url.join(&endpoint).map_err(|err| {
            let err = err.to_string();
            Error::TaskEndHttpClient(err)
        })?;
        let response = match self.request {
            MsgRequest::Get { .. } => self.client.get(full_url).send().await,
            MsgRequest::Put { body, .. } => self.client.put(full_url).body(body).send().await,
            MsgRequest::Post { body, .. } => self.client.post(full_url).body(body).send().await,
        };

        let msg_response = single_request(endpoint.to_string(), response).await;

        self.output.send(msg_response).await.unwrap();

        Ok(())
    }
}

fn single_request(
    endpoint: String,
    response: std::result::Result<Response, reqwest::Error>,
) -> MsgResponse {
    let response = match response {
        Ok(v) => v,
        Err(e) => {
            return MsgResponse::Error {
                endpoint,
                error: e.to_string(),
            }
        }
    };

    let status = response.status();
    let Ok(body) = response.bytes().await else {
        return MsgResponse::Error {
            endpoint,
            error: "Failed to read response body".to_string(),
        };
    };
    if status != StatusCode::OK {
        let error = format!(
            "Error on request.\nRequest params: {:?}\nResponse text: {:?}",
            endpoint,
            String::from_utf8_lossy(&body)
        );
        error!("{}", error);
        return MsgResponse::Error { endpoint, error };
    }

    MsgResponse::Success {
        endpoint,
        body: body.to_vec(),
    }
}
