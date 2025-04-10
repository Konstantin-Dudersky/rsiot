use super::RequestResponseBound;

#[derive(Debug)]
pub struct FieldbusRequestWithIndex<TFieldbusRequest>
where
    TFieldbusRequest: RequestResponseBound,
{
    pub device_index: usize,
    pub request: TFieldbusRequest,
}
