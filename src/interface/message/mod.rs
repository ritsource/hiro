mod helpers;
pub mod payload;

use std::io;
use std::mem;

#[allow(unused_imports)]
use super::data;
use crate::constants;
pub use helpers::gen_buf_for_rpc;

pub type ProtocolID = [u8; 20];

pub type MessagePayloadLength = u32;

pub type MessageID = u16;

pub enum Message {
  Ping,
  Error,
  FileReq,
  FileRes,
}

impl Default for Message {
  fn default() -> Self {
    Self::Ping
  }
}

impl Message {
  pub fn id(self) -> MessageID {
    match self {
      Self::Ping => 1,
      Self::Error => 2,
      Self::FileReq => 3,
      Self::FileRes => 4,
    }
  }

  // NOTE: return Result<Self> instead of Option<Self>
  pub fn from_id(id: MessageID) -> Option<Self> {
    Some(match id {
      1 => Self::Ping,
      2 => Self::Error,
      3 => Self::FileReq,
      4 => Self::FileRes,
      _ => return None,
    })
  }
}

pub struct Header {
  proto_id: ProtocolID,
  message_id: MessageID,
  payload_len: MessagePayloadLength,
  // pub from: data::Peer,
  // pub to: data:: Peer,
}

#[allow(dead_code)]
impl Header {
  pub fn new(message_id: MessageID, payload_len: MessagePayloadLength) -> Self {
    Self {
      proto_id: constants::PROTOCOL_IDENTIFIER_V1,
      message_id: message_id,
      payload_len: payload_len,
    }
  }

  pub fn from_reader<R>(mut reader: R) -> Result<(Self, R), (io::Error, R)>
  where
    R: io::Read + io::Write,
  {
    use constants::PROTOCOL_IDENTIFIER_V1;

    let mut buf = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
    if let Err(err) = reader.read(&mut buf) {
      return Err((err, reader));
    }
    let mut proto_id = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
    proto_id.copy_from_slice(&buf);

    let mut buf = [0u8; mem::size_of::<MessageID>()];
    if let Err(err) = reader.read(&mut buf) {
      return Err((err, reader));
    }
    let message_id = MessageID::from_be_bytes(buf);

    let mut buf = [0u8; mem::size_of::<MessagePayloadLength>()];
    if let Err(err) = reader.read(&mut buf) {
      return Err((err, reader));
    }
    let payload_len = MessagePayloadLength::from_be_bytes(buf);

    Ok((Self::new(message_id, payload_len), reader))
  }

  pub fn to_vec(self) -> Vec<u8> {
    let mut buf = constants::PROTOCOL_IDENTIFIER_V1.to_vec();
    buf.extend(self.message_id().to_be_bytes().iter());
    buf.extend(self.payload_len().to_be_bytes().iter());
    buf
  }

  pub fn proto_id(&self) -> ProtocolID {
    self.proto_id
  }

  pub fn message_id(&self) -> MessageID {
    self.message_id
  }

  pub fn payload_len(&self) -> MessagePayloadLength {
    self.payload_len
  }
}
