use rs

use crate::function::function;

#[derive(Clone)]
pub struct Config {
    /// Уровень логгирования
    pub level: Level,
}

pub fn create<TMessage>(config: Config) -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, function);
    Box::new(cmp)
}
