//! cargo run -p rsiot --example cmp_mqtt_client --features "cmp_mqtt_client" --target="x86_64-unknown-linux-gnu"

#[cfg(feature = "cmp_mqtt_client")]
mod message;
#[cfg(feature = "cmp_mqtt_client")]
mod publish;
#[cfg(feature = "cmp_mqtt_client")]
mod subscribe;

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
