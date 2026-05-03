use tokio::sync::mpsc;

use crate::components_config::master_device::{FieldbusRequestWithIndex, RequestResponseBound};

pub struct AddIndex<TFieldbusRequest, TError>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub input: mpsc::Receiver<TFieldbusRequest>,
    pub output: mpsc::Sender<FieldbusRequestWithIndex<TFieldbusRequest>>,
    pub device_index: usize,
    pub error_tokiompscsend: fn() -> TError,
}

impl<TFieldbusRequest, TError> AddIndex<TFieldbusRequest, TError>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub async fn spawn(mut self) -> Result<(), TError> {
        while let Some(request) = self.input.recv().await {
            let request_with_index = FieldbusRequestWithIndex {
                device_index: self.device_index,
                request,
            };
            self.output
                .send(request_with_index)
                .await
                .map_err(|_| (self.error_tokiompscsend)())?;
        }
        Ok(())
    }
}
