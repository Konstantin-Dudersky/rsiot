#[derive(Clone, Debug)]
pub struct IdStandard(u16);

impl IdStandard {
    pub fn as_raw(&self) -> u16 {
        self.0
    }
}
