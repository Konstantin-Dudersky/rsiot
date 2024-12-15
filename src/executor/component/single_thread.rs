use async_trait::async_trait;

use crate::message::{MsgDataBound, ServiceBound};

use super::super::{CmpInOut, CmpResult, ComponentError};

/// Представление обобщенного компонента
pub struct Component<TConfig, TMessage, TService>
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
{
    in_out: Option<CmpInOut<TMessage, TService>>,
    config: Option<TConfig>,
}

impl<TConfig, TMsg, TService> Component<TConfig, TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Создание компонента
    pub fn new(config: impl Into<TConfig>) -> Self {
        Self {
            in_out: None,
            config: Some(config.into()),
        }
    }
}

#[async_trait(?Send)]
impl<TConfig, TMsg, TService> IComponent<TMsg, TService> for Component<TConfig, TMsg, TService>
where
    Self: IComponentProcess<TConfig, TMsg, TService>,
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    fn set_interface(&mut self, in_out: CmpInOut<TMsg, TService>) {
        self.in_out = Some(in_out);
    }

    async fn spawn(&mut self) -> CmpResult {
        let in_out = self
            .in_out
            .take()
            .ok_or(ComponentError::Initialization("input not set".into()))?;

        let config = self
            .config
            .take()
            .ok_or(ComponentError::Initialization("config not set".into()))?;

        self.process(config, in_out).await
    }
}

/// Трейт основной функции компонента
///
/// Каждый компонент должен определить данный трейт
#[async_trait(?Send)]
pub trait IComponentProcess<TConfig, TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Основная функция компонента
    async fn process(&self, config: TConfig, in_out: CmpInOut<TMsg, TService>) -> CmpResult;
}

/// Интерфейс компонента, который используется исполнитель при добавлении компонентов
#[async_trait(?Send)]
pub trait IComponent<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    fn set_interface(&mut self, in_out: CmpInOut<TMsg, TService>);

    async fn spawn(&mut self) -> CmpResult;
}
