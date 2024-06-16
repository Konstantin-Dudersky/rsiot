//! Компонент для вывода SVG-файла с возможностью изменения свойств элементов в зависимости от
//! сигналов

mod change_svg_prop;
mod component;
mod set_global_style;
mod svg_input;
mod svg_output;

pub use component::SvgDynamic;
pub use svg_input::SvgInput;
pub use svg_output::SvgOutput;

const INK_LABEL: &str = "inkscape:label";
