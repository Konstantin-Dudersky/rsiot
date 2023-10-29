pub use messages_lib::IMessage;

#[derive(Debug)]
pub enum Messages {
    Reg0(f64),
}

impl IMessage for Messages {}
