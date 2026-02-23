use esp_idf_svc::hal::can::config::Timing;

use crate::components_config::can_general::CanSettingsBitrate;

impl From<CanSettingsBitrate> for Timing {
    fn from(value: CanSettingsBitrate) -> Self {
        match value {
            CanSettingsBitrate::Standard {
                bitrate,
                sample_point: _,
            } => match bitrate {
                25_000 => Timing::B25K,
                50_000 => Timing::B50K,
                100_000 => Timing::B100K,
                125_000 => Timing::B125K,
                250_000 => Timing::B250K,
                500_000 => Timing::B500K,
                800_000 => Timing::B800K,
                1_000_000 => Timing::B1M,
                _ => panic!("Unsupported bitrate: {:?}", value),
            },
            CanSettingsBitrate::Custom {
                tq: _,
                prop_seg: _,
                phase_seg1: _,
                phase_seg2: _,
                sjw: _,
            } => panic!("Unsupported bitrate: {:?}", value),
        }
    }
}
