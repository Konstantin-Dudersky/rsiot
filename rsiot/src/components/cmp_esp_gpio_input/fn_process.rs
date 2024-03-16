use esp_idf_svc::hal::gpio::{InputPin, Level};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TPin, TMsg>(
    mut config: Config<TPin, TMsg>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TPin: InputPin,
{
    loop {
        let level = config.driver.get_level();
        let level = gpio_level_to_bool(&level);
        let msg = (config.fn_output)(level);
        in_out.send_output(msg).await.unwrap();
        config.driver.wait_for_any_edge().await.unwrap();
    }
}

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => true,
        Level::High => false,
    }
}
