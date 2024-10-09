#[cfg(feature = "cmp_plc")]
#[allow(dead_code, unused_variables)]
fn main() {
    // service -------------------------------------------------------------------------------------

    use rsiot::message::ServiceBound;

    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Service {
        cmp_esp_example,
    }

    impl ServiceBound for Service {}

    // messages ------------------------------------------------------------------------------------
    use rsiot::message::MsgDataBound;

    use rsiot::message::TimeToLiveValue;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum Custom {
        ExampleGroup(ExampleGroup),
    }

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum ExampleGroup {}

    impl MsgDataBound for Custom {
        type TService = Service;

        fn define_enabled_routes(&self) -> Vec<(Option<Self::TService>, Option<Self::TService>)> {
            vec![]
        }

        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    // fb_main -------------------------------------------------------------------------------------

    mod fb_main {
        use serde::Serialize;

        use rsiot::components::cmp_plc::plc::{FbSystemData, FunctionBlockBase, IFunctionBlock};

        #[derive(Clone, Default, Serialize)]
        pub struct I {}

        #[derive(Clone, Default, Serialize)]
        pub struct Q {}

        #[derive(Clone, Default, Serialize)]
        pub struct S {}

        impl IFunctionBlock<I, Q, S> for FunctionBlockBase<I, Q, S> {
            fn logic(input: &mut I, stat: &mut S, _system_data: &FbSystemData) -> Q {
                Q {}
            }
        }

        pub type FB = FunctionBlockBase<I, Q, S>;
    }

    // ANCHOR: config_plc_template

    use std::time::Duration;

    use rsiot::{components::cmp_plc, message::Message};

    pub fn config() -> cmp_plc::Config<Custom, fb_main::I, fb_main::Q, fb_main::S> {
        cmp_plc::Config {
            fn_cycle_init,
            fn_input,
            fn_output,
            fb_main: fb_main::FB::new(),
            period: Duration::from_millis(200),
            retention: None,
        }
    }

    fn fn_cycle_init(input: &mut fb_main::I) {}

    fn fn_input(input: &mut fb_main::I, msg: &Message<Custom>) {}

    fn fn_output(output: &fb_main::Q) -> Vec<Message<Custom>> {
        let msgs = vec![];

        msgs.into_iter()
            .map(|m| Message::new_custom(Custom::ExampleGroup(m)))
            .collect()
    }

    // ANCHOR_END: config_plc_template
}

#[cfg(not(feature = "cmp_plc"))]
fn main() {}
