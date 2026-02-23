use crate::components_config::can_general::CanId;

impl From<embedded_can::Id> for CanId {
    fn from(value: embedded_can::Id) -> Self {
        match value {
            embedded_can::Id::Standard(v) => CanId::Standard(v.as_raw()),
            embedded_can::Id::Extended(v) => CanId::Extended(v.as_raw()),
        }
    }
}
