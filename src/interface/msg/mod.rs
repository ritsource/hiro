pub mod types;

use super::data;

pub type MessagePayloadLength = u32;

pub type MessageTypeID = u16;

pub enum MessageType {
  PiecesAndPeersForFileRequest,
  PiecesAndPeersForFileResponse,
  Ping,
}

impl Default for MessageType {
  fn default() -> Self {
    MessageType::Ping
  }
}

pub fn build_message_buffer(msg_type: MessageType, payload: Vec<u8>) -> Vec<u8> {
  let mut buf = super::constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(message_type_id_from_message_type(msg_type).to_be_bytes().iter());
  buf.extend(((payload.len()) as MessagePayloadLength).to_be_bytes().iter());
  buf.extend(payload);
  buf
}

pub fn message_type_from_message_type_id(message_type_id: MessageTypeID) -> MessageType {
  match message_type_id {
    1 => MessageType::PiecesAndPeersForFileRequest,
    2 => MessageType::PiecesAndPeersForFileResponse,
    _ => MessageType::Ping,
  }
}

pub fn message_type_id_from_message_type(message_type: MessageType) -> MessageTypeID {
  match message_type {
    MessageType::PiecesAndPeersForFileRequest => 1,
    MessageType::PiecesAndPeersForFileResponse => 2,
    MessageType::Ping => 0,
  }
}
