// TODO - Implement timeout

use gloo::net::{
    http::{Request, Response},
    Error as GlooError,
};
use http::StatusCode;
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
}

impl HttpClient {
    pub async fn spawn(mut self) -> Result<()> {
        // Парсим url
        let base_url = Url::parse(&self.base_url).map_err(|err| {
            let err = err.to_string();
            let err = format!("Cannot parse url: {}", err);
            Error::TaskEndHttpClient(err)
        })?;

        let mut task_set = JoinSet::new();

        while let Some(request) = self.input.recv().await {
            let task = SingleRequest {
                output: self.output.clone(),
                base_url: base_url.clone(),
                request,
            };
            join_set_spawn(&mut task_set, "cmp_http_client_wasm", task.spawn());
        }

        Err(Error::TaskEndHttpClient("".into()))
    }
}

pub struct SingleRequest {
    pub output: mpsc::Sender<MsgResponse>,
    pub base_url: Url,
    pub request: MsgRequest,
}

impl SingleRequest {
    pub async fn spawn(self) -> super::Result<()> {
        let endpoint = self.request.get_endpoint().to_string();
        let full_url = self.base_url.join(&endpoint).map_err(|err| {
            let err = err.to_string();
            Error::TaskEndHttpClient(err)
        })?;
        let full_url = full_url.to_string();
        let response = match self.request {
            MsgRequest::Get { .. } => Request::get(&full_url).send().await,
            MsgRequest::Put { body, .. } => {
                Request::put(&full_url)
                    .body(body)
                    .map_err(|e| Error::TaskEndHttpClient(e.to_string()))?
                    .send()
                    .await
            }
            MsgRequest::Post { body, .. } => {
                Request::post(&full_url)
                    .body(body)
                    .map_err(|e| Error::TaskEndHttpClient(e.to_string()))?
                    .send()
                    .await
            }
        };

        let msg_response = process_response(endpoint.to_string(), response).await;

        self.output.send(msg_response).await.unwrap();

        Ok(())
    }
}

async fn process_response(
    endpoint: String,
    response: std::result::Result<Response, GlooError>,
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

    let Ok(body) = response.binary().await else {
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
