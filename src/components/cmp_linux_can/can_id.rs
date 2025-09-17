use super::{CanId, Error};

impl TryFrom<CanId> for socketcan::CanId {
    type Error = Error;

    fn try_from(value: CanId) -> Result<Self, Self::Error> {
        match value {
            CanId::Standard(id) => {
                let id =
                    socketcan::StandardId::new(id).ok_or_else(|| Error::InvalidId(id as u64))?;
                Ok(socketcan::CanId::Standard(id))
            }
            CanId::Extended(id) => {
                let id =
                    socketcan::ExtendedId::new(id).ok_or_else(|| Error::InvalidId(id as u64))?;
                Ok(socketcan::CanId::Extended(id))
            }
        }
    }
}

impl TryFrom<socketcan::CanId> for CanId {
    type Error = Error;

    fn try_from(value: socketcan::CanId) -> Result<Self, Self::Error> {
        match value {
            socketcan::CanId::Standard(id) => {
                let id = id.as_raw();
                Ok(CanId::Standard(id))
            }
            socketcan::CanId::Extended(id) => {
                let id = id.as_raw();
                Ok(CanId::Extended(id))
            }
        }
    }
}
