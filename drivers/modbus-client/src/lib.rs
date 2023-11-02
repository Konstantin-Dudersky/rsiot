use tokio::sync::mpsc::{Receiver, Sender};

use messages_lib::IMessage;

async fn client(
    channel_to_modbus: Receiver<impl IMessage>,
    channel_from_modbus: Sender<impl IMessage>,
) {
}
