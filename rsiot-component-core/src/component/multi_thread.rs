use async_trait::async_trait;

use crate::{CmpInOut, ComponentError};

pub struct Component<TConfig, TMsg> {
    in_out: Option<CmpInOut<TMsg>>,
    config: Option<TConfig>,
}

impl<TConfig, TMsg> Component<TConfig, TMsg> {
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
    TMsg: Send + Sync,
    Self: IComponentProcess<TConfig, TMsg>,
    TConfig: Send,
{
    fn set_interface(&mut self, in_out: CmpInOut<TMsg>) {
        self.in_out = Some(in_out);
    }

    async fn spawn(&mut self) -> Result<(), ComponentError> {
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

#[async_trait]
pub trait IComponentProcess<TConfig, TMsg> {
    async fn process(&self, config: TConfig, in_out: CmpInOut<TMsg>) -> Result<(), ComponentError>;
}

#[async_trait]
pub trait IComponent<TMsg> {
    fn set_interface(&mut self, in_out: CmpInOut<TMsg>);

    async fn spawn(&mut self) -> Result<(), ComponentError>;
}
