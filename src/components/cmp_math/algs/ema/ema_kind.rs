/// Разновидность алгоритма
#[derive(Clone, Copy)]
pub enum EmaKind {
    /// По последнему значению
    Last,

    /// По следующему значению
    Next,

    /// Линейный
    Linear,
}
