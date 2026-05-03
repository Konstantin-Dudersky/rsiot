#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Wrong address in packet: {address_in_packet}; expected: {expected_address}")]
    AddressMismatch {
        address_in_packet: u8,
        expected_address: u8,
    },

    #[error("CRC mismatch")]
    CRCMismatch,

    #[error("Length mismatch in packet: {length_in_packet}; expected: {expected_length}")]
    LengthMismatch {
        length_in_packet: usize,
        expected_length: usize,
    },

    #[error("Frame too short. Length: {length}")]
    FrameTooShort { length: usize },

    #[error("TryGetError: {0}")]
    TryGetError(#[from] bytes::TryGetError),
}
