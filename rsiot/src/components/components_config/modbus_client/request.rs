/// Параметры запроса Modbus
#[derive(Clone, Debug)]
pub enum Request {
    /// (start address, count)
    ReadCoils(u16, u16),
    /// (start address, count)
    ReadHoldingRegisters(u16, u16),
    /// (address, value)
    WriteSingleRegister(u16, u16),
}
