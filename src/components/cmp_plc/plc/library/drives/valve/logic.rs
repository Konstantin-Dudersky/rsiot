use super::super::mode_select;

use super::{IHmiCommand, QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

pub fn logic(input: &I, stat: &mut S) -> Q {
    // Выбор режима
    stat.mode.call(mode_select::I {
        mode_plc_hmi: input.mode_plc_hmi,
        auto_mode_plc: input.auto_mode_plc,
        man_mode_plc: input.man_mode_plc,
        local_mode_plc: false,
        oos_mode_plc: false,
        hmi_command: match input.hmi_command {
            IHmiCommand::NoCommand => mode_select::IHmiCommand::NoCommand,

            IHmiCommand::ManMode => mode_select::IHmiCommand::ManMode,
            IHmiCommand::AutoMode => mode_select::IHmiCommand::AutoMode,
            IHmiCommand::LocalMode => mode_select::IHmiCommand::LocalMode,
            IHmiCommand::OosMode => mode_select::IHmiCommand::OosMode,

            IHmiCommand::CloseMan | IHmiCommand::OpenMan => mode_select::IHmiCommand::NoCommand,
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
                auto_mode: stat.mode.output.hmi_status.hmi_permission.auto_mode,
                man_mode: stat.mode.output.hmi_status.hmi_permission.man_mode,
                local_mode: stat.mode.output.hmi_status.hmi_permission.local_mode,
                oos_mode: stat.mode.output.hmi_status.hmi_permission.oos_mode,
            },
        },
    }
}
