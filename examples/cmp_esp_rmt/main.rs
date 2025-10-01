#[cfg(feature = "cmp_esp")]
mod message;

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use esp_idf_svc::{
        hal::{
            prelude::Peripherals,
            rmt::{
                FixedLengthSignal, PinState, Pulse, PulseTicks, TxRmtDriver,
                config::{Loop, TransmitConfig},
            },
        },
        sys::link_patches,
    };
    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
    };
    use tokio::{task::LocalSet, time::sleep};

    use message::*;
    use tracing::level_filters::LevelFilter;

    // ESP
    link_patches();

    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    // Prepare the config.
    let config = TransmitConfig::new().clock_divider(1);
    // .looping(Loop::Count(1000));

    // Retrieve the output pin and channel from peripherals.
    let peripherals = Peripherals::take().unwrap();
    let channel = peripherals.rmt.channel0;
    let pin = peripherals.pins.gpio3;

    // Create an RMT transmitter.
    let mut tx = TxRmtDriver::new(channel, pin, &config)?;

    // Prepare signal pulse signal to be sent.
    let low = Pulse::new(PinState::Low, PulseTicks::new(10)?);
    let high = Pulse::new(PinState::High, PulseTicks::new(10)?);

    loop {
        // Transmit the signal.
        let mut signal = FixedLengthSignal::<2>::new();
        signal.set(0, &(low, high))?;
        signal.set(1, &(high, low))?;
        tx.start(signal)?;
    }

    sleep(Duration::MAX).await;

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Msg>::new(executor_config)
            .wait_result()
            .await?;

        Ok(()) as anyhow::Result<()>
    });
    local_set.await;

    Ok(())
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {
    unimplemented!()
}
