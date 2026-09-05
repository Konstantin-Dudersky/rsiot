use bitvec::prelude::*;
use std::time::Duration;
use tracing::info;

use rsiot::{components::cmp_linux_can::*, executor::Component};

use super::messages::*;

pub fn cmp() -> Component<Config<Msg, impl Fn(&Msg) -> anyhow::Result<Option<Vec<CanFrame>>>>, Msg>
{
    let config = Config {
        ifname: "can1".into(),
        can_settings: CanSettings {
            bitrate: CanSettingsBitrate::Standard {
                bitrate: 1_000_000,
                sample_point: None,
            },
            dbitrate: CanSettingsDbitrate::None,
            mode_loopback: true,
            mode_listen_only: false,
            mode_triple_sampling: false,
            mode_one_shot: false,
            mode_berr_reporting: false,
            mode_fd: false,
            mode_presume_ack: false,
            mode_fd_non_iso: false,
            mode_cc_len8_dlc: false,
            mode_tdcv_mode: false,
            restart_ms: Some(1000),
        },
        fn_input: |msg: &Msg| {
            let mut data = [0u8; 8];

            let frame = match msg {
                Msg::Counter(v) => {
                    let bits = data.view_bits_mut::<Msb0>();
                    bits[32..48].store_be(*v);
                    CanFrame::Normal {
                        id: CanId::Extended(0x01),
                        data: data.to_vec(),
                    }
                }
            };

            Ok(Some(vec![frame]))
        },
        fn_output: |frame| {
            info!("Frame: {frame:?}");
            Ok(None)
        },
        filters: vec![CanFilter::Standard {
            id: 0b101,
            // mask: 0x1FFF_FFFF,
            mask: 0b10,
        }],
        async_version: false,
    };

    Cmp::new(config)
}
