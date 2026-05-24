use std::time::Duration;

use rsiot::{components::cmp_slint::*, executor::Component};
use slint::{ComponentHandle, Weak};

use crate::msg::*;

slint::include_modules!();

pub fn cmp(slint_window: Weak<MainWindow>) -> Component<Config<Msg, MainWindow>, Msg> {
    let config = Config {
        slint_window,
        fn_input: |_msg, w| {
            let input = w.global::<Input>();
            input.set_test_int(0)
        },
        fn_output: |w, sender| {
            let output = w.global::<Output>();

            let sender_clone = sender.clone();
            output.on_button_pressed(move || {
                let msg = Msg::ButtonPressed;
                sender_clone.send(msg);
            });
        },
        output_period: Duration::from_millis(100),
    };

    Cmp::new(config)
}
