/// Идентификатор CAN-кадра
#[derive(Clone, Copy, Debug)]
pub enum CanId {
    /// Стандартный идентификатор 11-битный
    Standard(u16),
    /// Расширенный идентификатор 29-битный
    Extended(u32),
}

impl CanId {
    /// Возвращает идентификатор в виде 32-битного числа
    pub fn as_raw(&self) -> u32 {
        match self {
            CanId::Standard(v) => *v as u32,
            CanId::Extended(v) => *v,
        }
    }
}

impl TryFrom<CanId> for embedded_can::Id {
    type Error = String;

    fn try_from(value: CanId) -> Result<Self, Self::Error> {
        match value {
            CanId::Standard(v) => {
                let id = embedded_can::StandardId::new(v);
                let Some(id) = id else {
                    return Err("Invalid standard ID".to_string());
                };

                Ok(embedded_can::Id::Standard(id))
            }
            CanId::Extended(v) => {
                let id = embedded_can::ExtendedId::new(v);
                let Some(id) = id else {
                    return Err("Invalid extended ID".to_string());
                };

                Ok(embedded_can::Id::Extended(id))
            }
        }
    }
}

impl From<embedded_can::Id> for CanId {
    fn from(value: embedded_can::Id) -> Self {
        match value {
            embedded_can::Id::Standard(v) => CanId::Standard(v.as_raw()),
            embedded_can::Id::Extended(v) => CanId::Extended(v.as_raw()),
        }
    }
}
