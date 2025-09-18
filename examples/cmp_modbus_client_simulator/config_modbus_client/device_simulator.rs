use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use rsiot::{
    components::cmp_modbus_client::{
        FieldbusRequest, FieldbusResponse, RequestContent, ResponseContent,
    },
    components_config::master_device::{
        self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait,
    },
    message::{Message, MsgDataBound},
};
use strum::FromRepr;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub update_period: Duration,
    pub fn_input: fn(&TMsg, &mut Buffer),
    pub fn_output: fn(&mut Buffer) -> Vec<TMsg>,
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, FieldbusRequest, FieldbusResponse> for Device<TMsg>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<TMsg, FieldbusRequest, FieldbusResponse, Buffer> = DeviceBase {
            fn_init_requests: |_| vec![],
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.update_period,
                fn_requests: |_| {
                    let input_registers = FieldbusRequest::new(
                        RequestKind::ReadHoldingRegisters,
                        RequestContent::ReadHoldingRegisters {
                            start_address: 0,
                            count: 3,
                        },
                    );
                    Ok(vec![input_registers])
                },
            }],
            fn_msgs_to_buffer: self.fn_input,
            fn_buffer_to_request: |buffer| {
                let req = RequestContent::WriteSingleRegister {
                    address: 0,
                    value: buffer.value_write as u16,
                };
                let req = FieldbusRequest::new(RequestKind::WriteValue, req);

                Ok(vec![req])
            },
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.into();

                match request_kind {
                    RequestKind::ReadHoldingRegisters => {
                        if let ResponseContent::WordVector(data) = response.response_content {
                            buffer.value_read = data[0] as f64;
                        }
                    }
                    _ => return Ok(false),
                }

                Ok(false)
            },
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer::default(),
        };
        device
            .spawn(
                "simulator",
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await
            .unwrap();
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub value_write: f64,
    pub value_read: f64,
}

impl BufferBound for Buffer {}

#[derive(FromRepr)]
pub enum RequestKind {
    WriteValue,
    ReadHoldingRegisters,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}

impl From<u8> for RequestKind {
    fn from(value: u8) -> Self {
        RequestKind::from_repr(value as usize).unwrap()
    }
}

// input_config: vec![InputConfig {
//     fn_input: |msg| match msg.get_custom_data()? {
//         Messages::ValueWrite(val) => Some(Request::WriteSingleRegister(0, val as u16)),
//         Messages::ValueRead(_) => None,
//     },
//     fn_on_success: |_data| vec![],
//     fn_on_failure: Vec::new,
// }],

// PeriodicConfig {
//     period: Duration::from_secs(2),
//     request: Request::ReadHoldingRegisters(0, 1),
//     fn_on_success: |data| {
//         let mut msgs = vec![];
//         if let Response::WordVector(data) = data {
//             msgs.push(Message::new_custom(Messages::ValueRead(data[0] as f64)));
//         }
//         msgs
//     },
//     fn_on_failure: Vec::new,
// }
