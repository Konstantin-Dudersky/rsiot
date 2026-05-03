use tokio::sync::mpsc;

use crate::components_config::master_device::{FieldbusResponseWithIndex, RequestResponseBound};

pub struct SplitResponses<TFieldbusResponse, TError>
where
    TFieldbusResponse: RequestResponseBound,
{
    pub input: mpsc::Receiver<FieldbusResponseWithIndex<TFieldbusResponse>>,
    pub output: Vec<mpsc::Sender<TFieldbusResponse>>,
    pub error_tokiompscsend: fn() -> TError,
}
impl<TFieldbusResponse, TError> SplitResponses<TFieldbusResponse, TError>
where
    TFieldbusResponse: RequestResponseBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Some(response_with_index) = self.input.recv().await {
            let device_index = response_with_index.device_index;
            let response = response_with_index.response;
            self.output[device_index]
                .send(response)
                .await
                .map_err(|_| (self.error_tokiompscsend)())?;
        }
        Ok(())
    }
}
