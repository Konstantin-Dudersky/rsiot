use super::{IdExtended, IdStandard};

#[derive(Clone, Debug)]
pub enum Id {
    Standard(IdStandard),
    Extended(IdExtended),
}
