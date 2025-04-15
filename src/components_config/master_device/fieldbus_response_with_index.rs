use super::RequestResponseBound;

#[derive(Debug)]
pub struct FieldbusResponseWithIndex<TFieldbusResponse>
where
    TFieldbusResponse: RequestResponseBound,
{
    pub device_index: usize,
    pub response: TFieldbusResponse,
}
