use tokio::{
    main, spawn,
    sync::{broadcast, mpsc},
    time::{sleep, Duration},
};

use rsiot_extra_components::cmpbase_mpsc_to_broadcast;
use rsiot_messages_core::ExampleMessage;
use tracing::info;

#[main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let (mpsc_send, mpsc_rcv) = mpsc::channel::<ExampleMessage>(128);

    let (broadcast_send, _) = broadcast::channel::<ExampleMessage>(128);
    let mut broadcast_rcv_1 = broadcast_send.subscribe();
    let mut broadcast_rcv_2 = broadcast_send.subscribe();

    let mut counter = 0.0;

    #[allow(unreachable_code)]
    let _source_task = spawn(async move {
        loop {
            let msg = ExampleMessage::ValueInstantF64(counter);
            counter += 1.0;
            mpsc_send.send(msg).await?;
            sleep(Duration::from_secs(2)).await;
        }
        Ok(()) as anyhow::Result<()>
    });

    let main_task = spawn(cmpbase_mpsc_to_broadcast::new(mpsc_rcv, broadcast_send));

    let _end_task_1 = spawn(async move {
        while let Ok(res) = broadcast_rcv_1.recv().await {
            info!("end_task_1: {:?}", res)
        }
    });

    let _end_task_2 = spawn(async move {
        while let Ok(res) = broadcast_rcv_2.recv().await {
            info!("end_task_2: {:?}", res)
        }
    });

    main_task.await??;
    Ok(())
}
