#[derive(Clone, Debug)]
pub struct IdExtended(u32);

impl IdExtended {
    pub fn as_raw(&self) -> u32 {
        self.0
    }
}
