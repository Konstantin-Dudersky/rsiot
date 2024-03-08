use async_trait::async_trait;

use super::super::{CmpInOut, ComponentError};

pub struct Component<TConfig, TMessage> {
    in_out: Option<CmpInOut<TMessage>>,
    config: Option<TConfig>,
}

impl<TConfig, TMsg> Component<TConfig, TMsg> {
    pub fn new(config: impl Into<TConfig>) -> Self {
        Self {
            in_out: None,
            config: Some(config.into()),
        }
    }
}

#[async_trait(?Send)]
impl<TConfig, TMsg> IComponent<TMsg> for Component<TConfig, TMsg>
where
    Self: IComponentProcess<TConfig, TMsg>,
{
    fn set_interface(&mut self, in_out: CmpInOut<TMsg>) {
        self.in_out = Some(in_out);
    }

    async fn spawn(&mut self) -> Result<(), ComponentError> {
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

#[async_trait(?Send)]
pub trait IComponentProcess<TConfig, TMsg> {
    async fn process(&self, config: TConfig, in_out: CmpInOut<TMsg>) -> Result<(), ComponentError>;
}

#[async_trait(?Send)]
pub trait IComponent<TMsg> {
    fn set_interface(&mut self, in_out: CmpInOut<TMsg>);

    async fn spawn(&mut self) -> Result<(), ComponentError>;
}
