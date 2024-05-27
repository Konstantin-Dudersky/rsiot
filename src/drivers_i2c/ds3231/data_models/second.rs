use bcd_numbers::BCD;
use bitvec::{order::Lsb0, view::BitView};

pub struct Second(u8);

impl Second {
    pub fn new_from_device(raw: u8) -> Self {
        let value: BCD<4> = BCD::new(raw as u128);
        println!("{:?}", value);

        // let raw_bits = raw.view_bits::<Lsb0>();

        // let value = 10
        //     * (8 * (raw_bits[7] as u8)
        //         + 4 * (raw_bits[6] as u8)
        //         + 2 * (raw_bits[5] as u8)
        //         + 1 * (raw_bits[4] as u8))
        //     + 8 * (raw_bits[3] as u8)
        //     + 4 * (raw_bits[2] as u8)
        //     + 2 * (raw_bits[1] as u8)
        //     + 1 * (raw_bits[0] as u8);
        Self(value.get_number() as u8)
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}
