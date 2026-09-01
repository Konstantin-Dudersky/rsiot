use std::{thread::sleep as std_thread_sleep, time::Duration};

use tokio::{sync::broadcast, time::sleep as tokio_time_sleep};
use tracing::warn;

use crate::components_config::can_general::{CanFrame, CanSettings, CanSettingsBitrate};

use super::{
    Error,
    can_socket::{CanSocketAsync, CanSocketSync},
};

pub struct SendToCanAsync {
    pub input: broadcast::Receiver<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
}
impl SendToCanAsync {
    pub async fn spawn(mut self) -> Result<(), Error> {
        let socket = CanSocketAsync::open(&self.ifname, self.can_settings.clone())?;
        // let delay_after_send = DelayAfterSend::new(&self.can_settings);
        while let Ok(frame) = self.input.recv().await {
            // let delay = delay_after_send.calc(&frame);
            let res = socket.write_frame(frame).await;
            // tokio_time_sleep(delay).await;
            if let Err(err) = res {
                warn!("Error sending frame: {}", err);
                break;
            }
        }
        Err(Error::TaskEndSendToCan)
    }
}

pub struct SendToCanSync {
    pub input: broadcast::Receiver<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
}
impl SendToCanSync {
    pub fn spawn(mut self) -> Result<(), Error> {
        let mut socket = CanSocketSync::open(&self.ifname, self.can_settings.clone())?;
        // let delay_after_send = DelayAfterSend::new(&self.can_settings);
        while let Ok(frame) = self.input.blocking_recv() {
            // let delay = delay_after_send.calc(&frame);
            let res = socket.transmit(frame);
            // std_thread_sleep(delay);
            if let Err(err) = res {
                warn!("Error sending frame: {}", err);
                break;
            }
        }
        Err(Error::TaskEndSendToCan)
    }
}

// Расчёт задержки после отправки кадра
// pub struct DelayAfterSend {
//     bitrate: f32,
// }
// impl DelayAfterSend {
//     pub fn new(can_settings: &CanSettings) -> Self {
//         let bitrate = match can_settings.bitrate {
//             CanSettingsBitrate::Standard {
//                 bitrate,
//                 sample_point: _,
//             } => bitrate as f32,
//             CanSettingsBitrate::Custom {
//                 tq: _,
//                 prop_seg: _,
//                 phase_seg1: _,
//                 phase_seg2: _,
//                 sjw: _,
//             } => todo!(),
//         };
//         Self { bitrate }
//     }

//     pub fn calc(&self, frame: &CanFrame) -> Duration {
//         let delay = frame.frame_size() / self.bitrate * 1.0;
//         Duration::from_secs_f32(delay)
//     }
// }
