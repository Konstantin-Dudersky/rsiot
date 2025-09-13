use super::Id;

#[derive(Clone, Debug)]
pub enum Frame {
    Normal { id: Id, data: Vec<u8> },
    // Remote(CanRemoteFrame),
    // Error(CanErrorFrame),
    // Fd(CanFdFrame),
}
