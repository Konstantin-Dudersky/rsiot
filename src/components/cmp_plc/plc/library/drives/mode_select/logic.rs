use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    match input.mode_plc_hmi {
        // Задание с hmi
        true => match input.hmi_command {
            IHmiCommand::AutoMode => stat.mode = QMode::Auto,
            IHmiCommand::ManMode => stat.mode = QMode::Manual,
            _ => (),
        },

        // Задание с plc
        false => {
            if input.auto_mode_plc {
                stat.mode = QMode::Auto;
            } else if input.man_mode_plc {
                stat.mode = QMode::Manual;
            }
        }
    }

    Q {
        mode: stat.mode,
        hmi_status: QHmiStatus {
            mode: stat.mode,
            hmi_permission: QHmiPermission {
                auto_mode: stat.mode != QMode::Auto && input.mode_plc_hmi,
                man_mode: stat.mode != QMode::Manual && input.mode_plc_hmi,
                local_mode: stat.mode != QMode::Local && input.mode_plc_hmi,
                oos_mode: stat.mode != QMode::Oos && input.mode_plc_hmi,
            },
        },
    }
}
