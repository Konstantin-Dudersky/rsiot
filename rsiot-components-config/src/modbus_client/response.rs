/// Ответ от устройства
#[derive(Clone, Debug)]
pub enum Response {
    U16(Vec<u16>),
    Bool(Vec<bool>),
}
