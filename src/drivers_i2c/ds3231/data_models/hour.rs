use bitvec::{order::Lsb0, view::BitView};

pub struct Hour(u8);

impl Hour {
    pub fn new_from_device(raw: u8) -> Self {
        let raw_bits = raw.view_bits::<Lsb0>();

        let hour_base = 10 * (raw_bits[4] as u8)
            + 8 * (raw_bits[3] as u8)
            + 4 * (raw_bits[2] as u8)
            + 2 * (raw_bits[1] as u8)
            + 1 * (raw_bits[0] as u8);
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

    pub fn get(&self) -> u8 {
        self.0
    }
}
