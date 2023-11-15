use rsiot_http_client_config::HttpClientConfig;
use rsiot_messages_core::IMessage;

pub async fn component_http_client<TMessage>(
    config: HttpClientConfig<TMessage>,
) -> ()
where
    TMessage: IMessage,
{
    let body = reqwest::get("http://localhost/get")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("body = {:?}", body);
}
