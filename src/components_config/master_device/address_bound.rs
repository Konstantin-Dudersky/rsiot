/// Ограничения для адреса подчиненных устройств
pub trait AddressBound
where
    Self: Clone + Copy + Default + PartialEq + Send + Sync,
{
}
