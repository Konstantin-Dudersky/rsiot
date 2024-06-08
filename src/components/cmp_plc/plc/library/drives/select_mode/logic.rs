use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    match input.mode_source {
        // Задание с hmi
        false => match input.hmi_command {
            IHmiCommand::mode_auto => stat.mode = QMode::Auto,
            IHmiCommand::mode_man => stat.mode = QMode::Manual,
            _ => (),
        },

        // Задание с plc
        true => {
            if input.mode_auto {
                stat.mode = QMode::Auto;
            } else if input.mode_man {
                stat.mode = QMode::Manual;
            }
        }
    }

    Q {
        mode: stat.mode,
        hmi_status: QHmiStatus {
            mode: stat.mode,
            hmi_permission: QHmiPermission {
                mode_auto: stat.mode != QMode::Auto && !input.mode_source,
                mode_man: stat.mode != QMode::Manual && !input.mode_source,
                mode_local: stat.mode != QMode::Local && !input.mode_source,
                mode_oos: stat.mode != QMode::Oos && !input.mode_source,
            },
        },
    }
}
