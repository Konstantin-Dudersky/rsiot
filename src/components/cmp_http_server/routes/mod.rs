mod get;
mod get_new;
mod list;
mod plc_input;
mod plc_output;
mod plc_static;
mod put;
mod replace;
mod root;

pub use {
    get::get, get_new::get_new, list::list, plc_input::plc_input, plc_output::plc_output,
    plc_static::plc_static, put::put, replace::replace, root::root,
};
