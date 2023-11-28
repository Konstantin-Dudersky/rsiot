use rsiot_macros::IntoEav;

use crate::eav::{Eav, IntoEav};

#[derive(IntoEav)]
enum TestMessage {
    Variant1(TestMessageType),
    Variant2(TestMessageType),
}

struct TestMessageType {
    value: f64,
}

impl IntoEav for TestMessageType {
    fn into_eav(&self) -> Vec<Eav> {
        vec![Eav::default()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = TestMessage::Variant1(TestMessageType { value: 34.7 });
        let eav = v.into_eav();
        println!("{:?}", eav);
    }
}
