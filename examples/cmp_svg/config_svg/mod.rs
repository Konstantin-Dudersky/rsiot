use rsiot::components::cmp_svg::*;

use crate::message::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg>, Msg> {
    let config = Config {
        file: include_str!("../files/input.svg"),
        fn_input: |msg| match msg {
            Msg::Counter(v) => {
                vec![SvgChange {
                    id: "rect1",
                    change: vec![SvgChangeType::ChangeAttr {
                        attr_name: "width",
                        new_value: "50".to_string(),
                    }],
                    change_childs: None,
                }]
            }
            _ => vec![],
        },
        fn_output: |svg| Msg::OutputSvg(svg.to_vec()),
    };

    Cmp::new(config)
}
