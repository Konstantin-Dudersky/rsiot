//! Пример реализации структуры идентификаторов CAN
//!
//! Используется расширенный формат шириной 29 бит. Стурктура идентификатора:
//! - 5 бит - (prio) - приоритет
//! - 8 бит - (src) - адрес отправителя
//! - 8 бит - (dst) - адрес получателя
//! - 8 бит - (type) - тип кадра. Между отправителем и получателем может быть много разных кадров

use bitvec::prelude::*;

use super::CanId;

#[derive(Debug, Default, PartialEq)]
/// Структура идентификатора CAN
pub struct CanIdHelper(u32);

impl CanIdHelper {
    /// Создать новый идентификатор CAN
    pub fn new(prio: u8, src: u8, dst: u8, type_: u8) -> Self {
        let mut value = 0;
        let bits = value.view_bits_mut::<Lsb0>();
        bits[24..29].store_be(prio);
        bits[16..24].store_be(src);
        bits[8..16].store_be(dst);
        bits[0..8].store_be(type_);
        Self(value)
    }

    /// Установить приоритет
    pub fn set_prio(self, prio: u8) -> Self {
        let mut value = self.0;
        let bits = value.view_bits_mut::<Lsb0>();
        bits[24..29].store_be(prio);
        Self(value)
    }

    /// Установить адрес отправителя
    pub fn set_src(self, src: u8) -> Self {
        let mut value = self.0;
        let bits = value.view_bits_mut::<Lsb0>();
        bits[16..24].store_be(src);
        Self(value)
    }

    /// Установить адрес получателя
    pub fn set_dst(self, dst: u8) -> Self {
        let mut value = self.0;
        let bits = value.view_bits_mut::<Lsb0>();
        bits[8..16].store_be(dst);
        Self(value)
    }

    /// Установить тип кадра
    pub fn set_type(self, type_: u8) -> Self {
        let mut value = self.0;
        let bits = value.view_bits_mut::<Lsb0>();
        bits[0..8].store_be(type_);
        Self(value)
    }
}

impl From<CanIdHelper> for u32 {
    fn from(id: CanIdHelper) -> Self {
        id.0
    }
}
impl From<&CanIdHelper> for u32 {
    fn from(id: &CanIdHelper) -> Self {
        id.0
    }
}
impl From<u32> for CanIdHelper {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<CanIdHelper> for CanId {
    fn from(value: CanIdHelper) -> Self {
        Self::Extended(value.0)
    }
}
impl TryFrom<CanId> for CanIdHelper {
    type Error = String;

    fn try_from(value: CanId) -> Result<Self, Self::Error> {
        match value {
            CanId::Standard(_) => Err("Standard id not supported".to_string()),
            CanId::Extended(v) => {
                let id: CanIdHelper = v.into();
                Ok(id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //! cargo test components_config::can_general::can_id_helper --features="cmp_linux_can" -- --nocapture

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_1() {
        let id = CanIdHelper::default().set_prio(1);
        assert_eq!(Into::<u32>::into(&id), 0x01000000);

        let id = id.set_src(120);
        assert_eq!(Into::<u32>::into(&id), 0x01780000);

        let id = id.set_dst(76);
        assert_eq!(Into::<u32>::into(&id), 0x01784C00);

        let id = id.set_type(40);
        assert_eq!(Into::<u32>::into(&id), 0x01784C28);
    }
}
