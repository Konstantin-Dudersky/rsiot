use super::{QHmiPermission, QHmiStatus, QMode, QState, I, Q, S};

use super::super::drives::mode_select;

pub fn logic(_input: &I, _stat: &mut S) -> Q {
    Q {
        hmi_status: QHmiStatus {
            state: QState::default(),
            mode: QMode::default(),
            hmi_permission: QHmiPermission::default(),
        },
    }
}
