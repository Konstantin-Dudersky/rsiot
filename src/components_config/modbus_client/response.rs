/// Ответ от устройства
#[derive(Clone, Debug)]
pub enum Response {
    /// Массив слов
    WordVector(Vec<u16>),

    /// Массив бит
    BitVector(Vec<bool>),

    /// Без ответа
    Unit,
}
