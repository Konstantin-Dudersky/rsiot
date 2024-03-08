use rsiot::message::{example_message::*, message_new, *};

fn main() {
    let value = 123.4;
    let s = message_new!("Custom-ValueInstantF64::value");
    println!("{:?}", s);
}
