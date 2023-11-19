use reqwest::{Client, Response, StatusCode};
use tokio::{
    spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::error;
use url::Url;

use rsiot_channel_utils::component_many_mpsc_to_mpsc;
use rsiot_http_client_config as hcc;
use rsiot_messages_core::IMessage;

use crate::{error::Error, periodic_runner::PeriodicRunner, types::Result_};

pub async fn component_http_client<TMessage>(
    stream_input: mpsc::Receiver<TMessage>,
    stream_output: mpsc::Sender<TMessage>,
    config: hcc::HttpClientConfig<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let url = config.connection_config.url.clone();

    let (stream_on_event_out, stream_output_1) = mpsc::channel::<TMessage>(100);
    let (stream_periodic_out, stream_output_2) = mpsc::channel::<TMessage>(100);

    // Задача для обработки запросов, на основе входящего потока сообщений
    let _task_on_event = spawn(process_on_event_requests(
        stream_input,
        stream_on_event_out,
        url.clone(),
        config.requests_on_event.clone(),
    ));

    // Задача для обработки периодических запросов
    let task_periodic = spawn(process_periodic_requests(
        stream_periodic_out,
        url.clone(),
        config.requests_periodic.clone(),
    ));

    // Задача для объединения нескольких потоков в выходной поток
    let _task_to_output = spawn(component_many_mpsc_to_mpsc(
        vec![stream_output_1, stream_output_2],
        stream_output,
    ));

    task_periodic.await.unwrap();
}

/// Обработка запросов, на основе входящего потока сообщений
async fn process_on_event_requests<TMessage>(
    mut stream_input: mpsc::Receiver<TMessage>,
    stream_output: mpsc::Sender<TMessage>,
    url: Url,
    requests: Vec<hcc::RequestOnEvent<TMessage>>,
) where
    TMessage: IMessage,
{
    while let Some(msg) = stream_input.recv().await {
        for req in &requests {
            let request_param = (req.condition)(msg.clone());
            let request_param = match request_param {
                Some(val) => val,
                None => continue,
            };
            let msgs = process_request_and_response(
                &url,
                &request_param,
                req.on_success,
                req.on_failure,
            )
            .await
            .unwrap();
            for msg in msgs {
                stream_output.send(msg).await.unwrap();
            }
        }
    }
}

/// Обработка периодических запросов
async fn process_periodic_requests<TMessage>(
    stream_output: mpsc::Sender<TMessage>,
    url: Url,
    requests: Vec<hcc::RequestPeriodic<TMessage>>,
) where
    TMessage: IMessage,
{
    let mut periodic_runner: Vec<PeriodicRunner> = requests
        .iter()
        .map(|r| PeriodicRunner::new(r.period))
        .collect();
    loop {
        let mut msgs_output: Vec<TMessage> = vec![];
        for (idx, period) in periodic_runner.iter_mut().enumerate() {
            if period.check() {
                let req = &requests[idx];
                let msgs = process_request_and_response(
                    &url,
                    &req.request_param,
                    req.on_success,
                    req.on_failure,
                )
                .await
                .unwrap();
                msgs_output.extend(msgs);
            }
        }
        for msg in msgs_output {
            stream_output.send(msg).await.unwrap();
        }
        sleep(Duration::from_millis(10)).await;
    }
}

/// Выполнение запроса и вызов коллбеков при ответе
async fn process_request_and_response<TMessage>(
    url: &Url,
    request_param: &hcc::RequestParam,
    on_success: hcc::CbkOnSuccess<TMessage>,
    on_failure: hcc::CbkOnFailure<TMessage>,
) -> Result_<Vec<TMessage>>
where
    TMessage: IMessage,
{
    let response = send_request(url.clone(), request_param).await;
    let response = match response {
        Ok(val) => val,
        Err(err) => match err {
            Error::ReqwestError(err) => {
                error!("{:?}", err);
                let msgs = (on_failure)();
                return Ok(msgs);
            }
            _ => return Err(err),
        },
    };
    let status = response.status();
    let text = response.text().await?;
    if status != StatusCode::OK {
        let msgs = (on_failure)();
        error!(
            "Error on request.\nRequest params: {:?}\nResponse text: {:?}",
            request_param, text
        );
        return Ok(msgs);
    }
    let msgs = (on_success)(text);
    Ok(msgs)
}

/// Выполнение запроса
async fn send_request(url: Url, req: &hcc::RequestParam) -> Result_<Response> {
    let endpoint = match req {
        hcc::RequestParam::Get(val) => val,
        hcc::RequestParam::Put(_) => todo!(),
        hcc::RequestParam::Post(_) => todo!(),
    };
    let url = url.join(endpoint).map_err(|err| {
        let err = err.to_string();
        Error::ConfigurationError(err)
    })?;
    let client = Client::new();
    let response = match req {
        hcc::RequestParam::Get(_) => client.get(url).send().await?,
        hcc::RequestParam::Put(_) => todo!(),
        hcc::RequestParam::Post(_) => todo!(),
    };
    Ok(response)
}
