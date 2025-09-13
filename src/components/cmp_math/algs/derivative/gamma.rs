// ANCHOR: Gamma
/// Коэффициент
#[derive(Clone, Copy)]
pub enum Gamma {
    /// Gamma = 0 - дифференциал (приращение)
    Differential,
    /// Произвольное значение
    Other(f64),
    /// Gamma = 1 - обычная производная
    Derivative,
}
// ANCHOR: Gamma

impl From<Gamma> for f64 {
    fn from(value: Gamma) -> Self {
        match value {
            Gamma::Differential => 0.0,
            Gamma::Other(gamma) => gamma,
            Gamma::Derivative => 1.0,
        }
    }
}
