use crate::{
    components::cmp_webstorage,
    message::{example_message::*, Message},
};

#[test]
fn fn_output() {
    // Ничего не загружать
    let fn_output_0 = |_| None;
    // Загружать все сообщения
    let fn_output_1 = |msg: Message<Custom>| Some(msg.clone());

    for fn_output in [fn_output_0, fn_output_1] {
        let _ = cmp_webstorage::Config::<Custom> {
            fn_output,
            ..Default::default()
        };
    }
}
