mod types;

use std::io;
use std::mem;
use std::net;

use std::io::prelude::*;

use super::data;
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

pub fn handle_stream(mut stream: net::TcpStream) -> Result<(), io::Error> {
  let mut buf = [0u8; mem::size_of::<MessageTypeID>()];
  stream.read(&mut buf)?;

  match message_type_from_message_type_id(MessageTypeID::from_be_bytes(buf)) {
    MessageType::PiecesAndPeersForFileRequest => {}
    MessageType::PiecesAndPeersForFileResponse => {}
    MessageType::None => {}
  }

  Ok(())
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
