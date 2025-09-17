/// Идентификатор CAN-кадра
#[derive(Clone, Debug)]
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
