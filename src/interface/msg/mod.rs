use std::io;

use super::data;

mod types;

pub use types::{PiecesAndPeersForFileRequest, PiecesAndPeersForFileResponse};

pub type MessageTypeID = u16;

pub enum MessageType {
  PiecesAndPeersForFileRequest,
  PiecesAndPeersForFileResponse,
  None,
}

impl Default for MessageType {
  fn default() -> Self {
    MessageType::None
  }
}

#[derive(Default)]
pub struct Message {
  pub message_type: MessageType,
  // pub sender: data::Peer,
  // pub receiver: data::Peer,
  pub payload: Vec<u8>,
}

impl Message {
  pub fn new(message_type: MessageType) -> Self {
    Self {
      message_type,
      ..Default::default()
    }
  }

  pub fn from_message_type_id(id: MessageTypeID) -> Self {
    Self::new(message_type_from_message_type_id(id))
  }
}

pub fn message_type_from_message_type_id(message_type_id: MessageTypeID) -> MessageType {
  match message_type_id {
    1 => MessageType::PiecesAndPeersForFileRequest,
    2 => MessageType::PiecesAndPeersForFileResponse,
    _ => MessageType::None,
  }
}

pub fn message_type_id_from_message_type(message_type: MessageType) -> MessageTypeID {
  match message_type {
    MessageType::PiecesAndPeersForFileRequest => 1,
    MessageType::PiecesAndPeersForFileResponse => 2,
    MessageType::None => 0,
  }
}
