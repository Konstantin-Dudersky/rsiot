use async_trait::async_trait;

use crate::message::MsgDataBound;

use super::super::{CmpInOut, CmpResult, ComponentError};

/// Представление обобщенного компонента
pub struct Component<TConfig, TMsg>
where
    TMsg: MsgDataBound,
{
    in_out: Option<CmpInOut<TMsg>>,
    config: Option<TConfig>,
}

impl<TConfig, TMsg> Component<TConfig, TMsg>
where
    TMsg: MsgDataBound,
{
    /// Создание компонента
    pub fn new(config: impl Into<TConfig>) -> Self {
        Self {
            config: Some(config.into()),
            in_out: None,
        }
    }
}

#[async_trait]
impl<TConfig, TMsg> IComponent<TMsg> for Component<TConfig, TMsg>
where
    TMsg: MsgDataBound,
    Self: IComponentProcess<TConfig, TMsg>,
    TConfig: Send,
{
    fn set_interface(&mut self, in_out: CmpInOut<TMsg>) {
        self.in_out = Some(in_out);
    }

    async fn spawn(&mut self) -> CmpResult {
        let config = self
            .config
            .take()
            .ok_or(ComponentError::Initialization("config not set".into()))?;

        let in_out = self
            .in_out
            .take()
            .ok_or(ComponentError::Initialization("in_out not set".into()))?;

        self.process(config, in_out).await
    }
}

/// Трейт основной функции компонента
///
/// Каждый компонент должен определить данный трейт
#[async_trait]
pub trait IComponentProcess<TConfig, TMsg>
where
    TMsg: MsgDataBound,
{
    /// Основная функция компонента
    async fn process(&self, config: TConfig, in_out: CmpInOut<TMsg>) -> CmpResult;
}

/// Интерфейс компонента, который используется исполнитель при добавлении компонентов
#[async_trait]
pub trait IComponent<TMsg>
where
    TMsg: MsgDataBound,
{
    fn set_interface(&mut self, in_out: CmpInOut<TMsg>);

    async fn spawn(&mut self) -> CmpResult;
}
