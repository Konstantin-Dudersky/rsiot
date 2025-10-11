/// Обработка событий svg-файла
#[derive(Clone, Default)]
pub struct SvgOutput<T>
where
    T: Fn(&str),
{
    /// Идентификаторы svg-элементов, нажатие на которые необходимо обрабатывать
    pub ids: Vec<String>,

    /// Функция обработки события нажатия на элемент.
    ///
    /// В качестве аргумента принимает id элемента, который был нажат
    pub callback: T,
}

impl<T> SvgOutput<T>
where
    T: Fn(&str),
{
    /// Создать структуру SvgOutput
    pub fn new(ids: Vec<&str>, callback: T) -> Self {
        Self {
            ids: ids.into_iter().map(String::from).collect(),
            callback,
        }
    }
}
