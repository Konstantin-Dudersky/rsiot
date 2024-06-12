//! Компонент для вывода SVG-файла с возможностью изменения свойств элементов в зависимости от
//! сигналов

mod component;
mod svg_input;
mod svg_output;

pub use component::SvgDynamic;
pub use svg_input::SvgInput;
pub use svg_output::SvgOutput;
