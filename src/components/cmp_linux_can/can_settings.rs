use tracing::info;

use super::{CanSettings, CanSettingsBitrate, CanSettingsDbitrate};

impl CanSettings {
    /// Преобразование настроек CAN в команду для ip link
    pub fn into_ip_link_command(&self, ifname: &str) -> Vec<String> {
        let mut command = CommandBuilder::new();

        command.push("ip");
        command.push("link");
        command.push("set");

        command.push(ifname);

        command.push("type");
        command.push("can");

        match self.bitrate {
            CanSettingsBitrate::Standard {
                bitrate,
                sample_point,
            } => {
                command.push("bitrate");
                command.push(bitrate);

                if let Some(sample_point) = sample_point {
                    command.push("sample-point");
                    command.push(sample_point);
                }
            }
            CanSettingsBitrate::Custom {
                tq,
                prop_seg,
                phase_seg1,
                phase_seg2,
                sjw,
            } => {
                command.push("tq");
                command.push(tq);

                command.push("prop-seg");
                command.push(prop_seg);

                command.push("phase-seg1");
                command.push(phase_seg1);

                command.push("phase-seg2");
                command.push(phase_seg2);

                if let Some(sjw) = sjw {
                    command.push("sjw");
                    command.push(sjw);
                }
            }
        }

        match self.dbitrate {
            CanSettingsDbitrate::None => (),
            CanSettingsDbitrate::Standard {
                dbitrate,
                dsample_point,
                tdcv: _,
                tdco: _,
                tdcf: _,
            } => {
                command.push("dbitrate");
                command.push(dbitrate);

                if let Some(dsample_point) = dsample_point {
                    command.push("dsample-point");
                    command.push(dsample_point);
                }
            }
            CanSettingsDbitrate::Custom {
                dtq: _,
                dprop_seg: _,
                dphase_seg1: _,
                dphase_seg2: _,
                dsjw: _,
                tdcv: _,
                tdco: _,
                tdcf: _,
            } => unimplemented!(),
        }

        command.push("loopback");
        command.push(mode_on_off(self.mode_loopback));

        command.push("listen-only");
        command.push(mode_on_off(self.mode_listen_only));

        command.push("triple-sampling");
        command.push(mode_on_off(self.mode_triple_sampling));

        command.push("one-shot");
        command.push(mode_on_off(self.mode_one_shot));

        command.push("berr-reporting");
        command.push(mode_on_off(self.mode_berr_reporting));

        command.push("fd");
        command.push(mode_on_off(self.mode_fd));

        command.push("fd-non-iso");
        command.push(mode_on_off(self.mode_fd_non_iso));

        command.push("presume-ack");
        command.push(mode_on_off(self.mode_presume_ack));

        command.push("cc-len8-dlc");
        command.push(mode_on_off(self.mode_cc_len8_dlc));

        if let Some(restart_ms) = self.restart_ms {
            command.push("restart-ms");
            command.push(restart_ms);
        }

        info!("CAN setup command: {:?}", command.join());

        command.build()
    }
}

struct CommandBuilder(Vec<String>);
impl CommandBuilder {
    fn new() -> Self {
        CommandBuilder(Vec::new())
    }
    fn push(&mut self, arg: impl ToString) {
        self.0.push(arg.to_string());
    }
    fn join(&self) -> String {
        self.0.join(" ")
    }
    fn build(self) -> Vec<String> {
        self.0
    }
}

fn mode_on_off(mode: bool) -> &'static str {
    if mode { "on" } else { "off" }
}
