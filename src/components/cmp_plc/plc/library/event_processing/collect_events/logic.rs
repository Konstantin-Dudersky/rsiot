use super::{QHmiStatus, I, Q, S};

pub fn logic(_input: &I, _stat: &mut S) -> Q {
    Q {
        hmi_status: QHmiStatus {},
    }
}
