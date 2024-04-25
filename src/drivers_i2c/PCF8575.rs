use crate::message::*;

pub struct PCF8575 {
    pub address: u8,
}

impl PCF8575 {
    pub fn new(address: u8) -> Self {
        PCF8575 { address }
    }

    pub fn set_input<TMsg>(pin: u8, fn_output: fn(bool) -> Option<Message<TMsg>>) {}

    pub fn periodic_request() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let pcf = PCF8575::new(1);
    }
}