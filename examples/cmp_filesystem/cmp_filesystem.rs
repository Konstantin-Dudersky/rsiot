//! Запуск:
//!
//! ```bash
//! cargo run --example cmp_filesystem --target x86_64-unknown-linux-gnu --features="cmp_filesystem"
//! ```

#[cfg(feature = "cmp_filesystem")]
#[tokio::main]
async fn main() {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_filesystem, cmp_inject_periodic, cmp_logger},
        executor::*,
        message::{example_message::*, example_service::*, *},
    };
    use tracing::Level;

    tracing_subscriber::fmt().init();

    // cmp_filesystem ------------------------------------------------------------------------------
    let config_filesystem = cmp_filesystem::Config {
        directory: "examples/cmp_filesystem/directory".into(),
        fn_input: |msg| {
            let msg1 = msg.get_custom_data();
            let Some(msg1) = msg1 else { return Ok(None) };
            match msg1 {
                Custom::SaveToFilesystem(_) => {
                    let key = format!("{}.json", msg.key);
                    let content = msg.serialize()?;
                    Ok(Some((key, content)))
                }
                _ => Ok(None),
            }
        },
        fn_output: |data| {
            let msg = Message::deserialize(data)?;
            Ok(Some(msg))
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter = 0;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_secs(5),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::SaveToFilesystem(counter));
            counter += 1;
            vec![msg]
        },
    };

    // config_logger -------------------------------------------------------------------------------
    let config_logger = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let text = msg.serialize()?;
            let text = format!("Header: {text}");
            Ok(Some(text))
        },
    };

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
    };
    ComponentExecutor::<Custom>::new(config_executor)
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .add_cmp(cmp_filesystem::Cmp::new(config_filesystem))
        .add_cmp(cmp_logger::Cmp::new(config_logger))
        .wait_result()
        .await
        .unwrap();
}

#[cfg(not(feature = "cmp_filesystem"))]
fn main() {
    unimplemented!()
}
