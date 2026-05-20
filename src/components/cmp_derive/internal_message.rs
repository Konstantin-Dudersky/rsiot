use super::BufferBound;

pub enum InternalMessage<TBuffer>
where
    TBuffer: BufferBound,
{
    BufferData(TBuffer),
    Period(()),
}
