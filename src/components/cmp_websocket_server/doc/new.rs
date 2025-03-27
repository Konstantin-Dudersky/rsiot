use crate::{components::cmp_websocket_server, message::Message};

use super::{client_to_server::*, messages::*, server_to_client::*};

pub fn new() -> crate::executor::Component<
    cmp_websocket_server::Config<Msg, ServerToClient, ClientToServer>,
    Msg,
> {
    let config = cmp_websocket_server::Config {
        port: 8011,
        fn_server_to_client: |msg: &Message<Msg>| {
            let msg = msg.get_custom_data()?;
            let s2c = match msg {
                Msg::ServerCounter(counter) => ServerToClient::ServerCounter(counter),
                _ => return None,
            };
            Some(s2c)
        },
        fn_client_to_server: |c2s: ClientToServer| {
            let msg = match c2s {
                ClientToServer::ClientCounter(counter) => {
                    Message::new_custom(Msg::CounterFromClient(counter))
                }
                _ => return vec![],
            };
            vec![msg]
        },
    };

    cmp_websocket_server::Cmp::new(config)
}
