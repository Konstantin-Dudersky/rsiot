use tokio::{
    sync::mpsc::Receiver,
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_messages_core::IMessage;

use crate::Error;

pub async fn start_timescaledb_storing<TMessage>(
    mut channel_rcv: Receiver<TMessage>,
    config: fn(TMessage) -> Option<()>,
) -> ()
where
    TMessage: IMessage,
{
    loop {
        info!("Start timescaledb-storing");
        let result = start_timescaledb_storing_loop::<TMessage>(
            &mut channel_rcv,
            config,
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn start_timescaledb_storing_loop<TMessage>(
    channel_rcv: &mut Receiver<TMessage>,
    config: fn(TMessage) -> Option<()>,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    Ok(())
}
