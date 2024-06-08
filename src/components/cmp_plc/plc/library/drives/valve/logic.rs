use super::super::select_mode;

use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    stat.mode.call(select_mode::I {
        mode_source: input.mode_plc_hmi,
        mode_auto: input.auto_mode_plc,
        mode_man: input.man_mode_plc,
        mode_local: false,
        mode_oos: false,
        hmi_command: match input.hmi_command {
            IHmiCommand::NoCommand => select_mode::IHmiCommand::no_command,

            IHmiCommand::ManMode => select_mode::IHmiCommand::mode_man,
            IHmiCommand::AutoMode => select_mode::IHmiCommand::mode_auto,
            IHmiCommand::LocalMode => select_mode::IHmiCommand::mode_local,
            IHmiCommand::OosMode => select_mode::IHmiCommand::mode_oos,

            IHmiCommand::CloseMan | IHmiCommand::OpenMan => select_mode::IHmiCommand::no_command,
        },
    });
    let mode = stat.mode.output.mode;

    Q {
        hmi_status: QHmiStatus {
            state: QState::default(),
            mode,
            hmi_permission: QHmiPermission {
                man_start: mode == QMode::Manual,
                man_stop: mode == QMode::Manual,
                auto_mode: stat.mode.output.hmi_status.hmi_permission.mode_auto,
                man_mode: stat.mode.output.hmi_status.hmi_permission.mode_man,
                local_mode: stat.mode.output.hmi_status.hmi_permission.mode_local,
                oos_mode: stat.mode.output.hmi_status.hmi_permission.mode_oos,
            },
        },
    }
}
