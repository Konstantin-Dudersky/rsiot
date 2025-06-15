//! docker run -d --name nanomq -p 1883:1883 -p 8083:8083 -p 8883:8883 emqx/nanomq:latest
//!
//! docker rm -f nanomq
//!
//! cargo run --example cmp_mqtt_client --features "cmp_mqtt_client, serde_json" --target="x86_64-unknown-linux-gnu"

#[cfg(feature = "cmp_mqtt_client")]
mod message;
#[cfg(feature = "cmp_mqtt_client")]
mod publish;
#[cfg(feature = "cmp_mqtt_client")]
mod subscribe;

#[cfg(feature = "cmp_mqtt_client")]
mod config_mqtt_server_publish;
#[cfg(feature = "cmp_mqtt_client")]
mod config_mqtt_server_subscribe;

#[cfg(feature = "cmp_mqtt_client")]
#[tokio::main]
async fn main() {
    use tokio::task::JoinSet;

    tracing_subscriber::fmt().init();

    let mut task_set = JoinSet::new();

    task_set.spawn(publish::publish());
    task_set.spawn(subscribe::subscribe());

    while let Some(res) = task_set.join_next().await {
        res.unwrap()
    }
}

#[cfg(not(feature = "cmp_mqtt_client"))]
fn main() {}
