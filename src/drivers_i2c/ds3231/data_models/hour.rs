use bitvec::{order::Lsb0, view::BitView};

pub struct Hour(u8);

impl Hour {
    pub fn new_from_bcd(raw: u8) -> Self {
        let raw_bits = raw.view_bits::<Lsb0>();

        let hour_base = 10 * (raw_bits[4] as u8)
            + 8 * (raw_bits[3] as u8)
            + 4 * (raw_bits[2] as u8)
            + 2 * (raw_bits[1] as u8)
            + (raw_bits[0] as u8);
        let hour_base = hour_base as i32;

        let hour_add = if raw_bits[6] {
            // 12-часовой формат
            if raw_bits[5] {
                // PM
                if hour_base != 12 {
                    12
                } else {
                    0
                }
            } else {
                // AM
                if hour_base == 12 {
                    -12
                } else {
                    0
                }
            }
        } else {
            // 24 часовой формат
            if raw_bits[5] {
                20
            } else {
                0
            }
        };

        let hour = (hour_base + hour_add) as u8;

        Self(hour)
    }

    pub fn new_from_dec(value: u8) -> Self {
        Self(value)
    }

    pub fn get_bcd(&self) -> u8 {
        let mut dec = self.0;

        let mut bcd = 0_u8;
        let bcd_bits = bcd.view_bits_mut::<Lsb0>();

        if dec >= 20 {
            bcd_bits.set(5, true);
            dec -= 20;
        }
        if dec >= 10 {
            bcd_bits.set(4, true);
            dec -= 10;
        }
        if dec >= 8 {
            bcd_bits.set(3, true);
            dec -= 8;
        }
        if dec >= 4 {
            bcd_bits.set(2, true);
            dec -= 4;
        }
        if dec >= 2 {
            bcd_bits.set(1, true);
            dec -= 2;
        }
        if dec >= 1 {
            bcd_bits.set(0, true);
        }
        bcd
    }

    pub fn get_dec(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Hour;

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features cmp_esp -- drivers_i2c::ds3231::data_models::hour::tests::test_bcd_to_dec --exact --show-output
    /// ```
    #[test]
    fn test_bcd_to_dec() {
        assert_eq!(0, Hour::new_from_bcd(0b0000_0000).get_dec());
        assert_eq!(1, Hour::new_from_bcd(0b0000_0001).get_dec());
        assert_eq!(8, Hour::new_from_bcd(0b0000_1000).get_dec());
        assert_eq!(10, Hour::new_from_bcd(0b0001_0000).get_dec());
        assert_eq!(12, Hour::new_from_bcd(0b0001_0010).get_dec());
        assert_eq!(13, Hour::new_from_bcd(0b0001_0011).get_dec());
        assert_eq!(20, Hour::new_from_bcd(0b0010_0000).get_dec());
        assert_eq!(23, Hour::new_from_bcd(0b0010_0011).get_dec());
    }

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features cmp_esp -- drivers_i2c::ds3231::data_models::hour::tests::test_dec_to_bcd --exact --show-output
    /// ```
    #[test]
    fn test_dec_to_bcd() {
        assert_eq!(0b0000_0000, Hour::new_from_dec(0).get_bcd());
        assert_eq!(0b0000_0001, Hour::new_from_dec(1).get_bcd());
        assert_eq!(0b0000_1000, Hour::new_from_dec(8).get_bcd());
        assert_eq!(0b0001_0000, Hour::new_from_dec(10).get_bcd());
        assert_eq!(0b0001_0010, Hour::new_from_dec(12).get_bcd());
        assert_eq!(0b0001_0011, Hour::new_from_dec(13).get_bcd());
        assert_eq!(0b0010_0000, Hour::new_from_dec(20).get_bcd());
        assert_eq!(0b0010_0011, Hour::new_from_dec(23).get_bcd());
    }
}
