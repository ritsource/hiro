use crate::interface::types;

pub const PROTOCOL_IDENTIFIER_V1: types::ProtocolIdentifier = *b"HIRO_PROTOCOL_ID_LOL";
pub const PROTOCOL_VERSION_V1: u8 = 1;

pub const MAX_MEMORIZABLE_PAYLOAD_SIZE: usize = 1024 * 1024 * 10; // 10mb
pub const DEFAULT_PIECE_SIZE: usize = 256;
// pub const DEFAULT_PIECE_SIZE: usize = 64 * 1024 * 1024;
