/// Тип аггрегации
#[derive(Debug, Clone, Default)]
pub enum AggType {
    /// Количество
    Count,
    /// Текущее значение, к которому не применялась аггрегация
    #[default]
    Current,
    /// Первое
    First,
    /// Инкремент
    Inc,
    /// Максимум
    Max,
    /// Среднее
    Mean,
    /// Минимум
    Min,
    /// Сумма
    Sum,
}
