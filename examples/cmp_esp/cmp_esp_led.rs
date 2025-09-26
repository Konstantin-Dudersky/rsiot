//! Example based on developer board ESP32-C3
//!
//! cargo run --example cmp_esp_led --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release

#[cfg(all(feature = "cmp_esp", feature = "log_esp"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::task::LocalSet;
    use tracing::{Level, level_filters::LevelFilter};

    use rsiot::{
        components::{cmp_esp_led, cmp_inject_periodic, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
        message::*,
    };

    link_patches();
    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
    pub enum Custom {
        LedColor(Vec<(u8, cmp_esp_led::ConfigRgb)>),
    }

    impl MsgDataBound for Custom {
        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    let peripherals = Peripherals::take().unwrap();

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Custom::LedColor(content) => format!("{content:?}"),
            };

            Ok(Some(text))
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(1000),
        fn_periodic: move || {
            let random = [0u8; 3];

            let one_led = cmp_esp_led::ConfigRgb {
                r: random[0],
                g: random[1],
                b: random[2],
            };
            let all_leds = vec![(10, one_led)];

            let msg = Custom::LedColor(all_leds);
            // let msg = Message::new_custom(Custom::LedColor(vec![cmp_esp_led::ConfigRgb {
            //     r: 255,
            //     g: 0,
            //     b: 0,
            // }]));
            vec![msg]
        },
    };

    // cmp_esp_led ---------------------------------------------------------------------------------
    let config_esp_led = cmp_esp_led::Config {
        pin: peripherals.pins.gpio0.into(),
        rmt_channel: peripherals.rmt.channel0,
        fn_input: |msg| {
            let msg = msg.get_custom_data()?;
            match msg {
                Custom::LedColor(config_rgb) => Some(config_rgb),
            }
        },
    };

    // executor ------------------------------------------------------------------------------------

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_esp_led::Cmp::new(config_esp_led))
            .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}

#[cfg(not(all(feature = "cmp_esp", feature = "log_esp")))]
fn main() {
    unimplemented!()
}
