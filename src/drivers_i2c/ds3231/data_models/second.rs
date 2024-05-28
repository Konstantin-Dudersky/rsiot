pub struct Second(u8);

impl Second {
    pub fn new_from_bcd(raw: u8) -> Self {
        let s = format!("{:x}", raw);
        let v = s.parse::<u8>().unwrap();
        Self(v)
    }

    pub fn new_from_dec(value: u8) -> Self {
        Self(value)
    }

    pub fn get_dec(&self) -> u8 {
        self.0
    }

    pub fn get_bcd(&self) -> u8 {
        let s = format!("{}", self.0);
        let i = u8::from_str_radix(&s, 16).unwrap();
        i
    }
}

#[cfg(test)]
mod tests {
    use super::Second;

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features cmp_esp -- drivers_i2c::ds3231::data_models::second::tests::test_bcd_to_dec --exact --show-output
    /// ```
    #[test]
    fn test_bcd_to_dec() {
        assert_eq!(0, Second::new_from_bcd(0x00).get_dec());
        assert_eq!(1, Second::new_from_bcd(0x01).get_dec());
        assert_eq!(10, Second::new_from_bcd(0x10).get_dec());
        assert_eq!(11, Second::new_from_bcd(0x11).get_dec());
        assert_eq!(59, Second::new_from_bcd(0x59).get_dec());
        assert_eq!(37, Second::new_from_bcd(0x37).get_dec());
    }

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features cmp_esp -- drivers_i2c::ds3231::data_models::second::tests::test_dec_to_bcd --exact --show-output
    /// ```
    #[test]
    fn test_dec_to_bcd() {
        assert_eq!(0x00, Second::new_from_dec(0).get_bcd());
        assert_eq!(0x01, Second::new_from_dec(1).get_bcd());
        assert_eq!(0x10, Second::new_from_dec(10).get_bcd());
        assert_eq!(0x11, Second::new_from_dec(11).get_bcd());
        assert_eq!(0x59, Second::new_from_dec(59).get_bcd());
        assert_eq!(0x37, Second::new_from_dec(37).get_bcd());
    }
}
