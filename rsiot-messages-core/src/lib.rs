use std::fmt::Debug;

pub trait IMessage
where
    Self: Clone + Debug + Send,
{
    /// Ключ для сохранения в базе данных
    fn key(&self) -> String {
        let full_str = format!("{:?}", self);
        let parenth_index = full_str.find('(');
        let full_str: String = match parenth_index {
            Some(value) => full_str.chars().take(value).collect(),
            None => full_str,
        };
        full_str
    }
}
