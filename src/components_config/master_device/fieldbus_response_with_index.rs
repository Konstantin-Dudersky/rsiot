use super::RequestResponseBound;

pub struct FieldbusResponseWithIndex<TFieldbusResponse>
where
    TFieldbusResponse: RequestResponseBound,
{
    pub device_index: usize,
    pub response: TFieldbusResponse,
}
