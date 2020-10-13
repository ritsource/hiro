mod helpers;

use std::io;
use std::mem;

pub use helpers::{gen_buf_for_rpc, new_message_buffer};

pub type MsgPayloadLen = u32;

pub type MsgTypeID = u16;

#[derive(Debug, Clone)]
pub enum MsgType {
  Ping,
  Error,
  FileReq,
  FileRes,
  PieceUploadReq,
  PieceUploadRes,
}

impl Default for MsgType {
  fn default() -> Self {
    Self::Ping
  }
}

impl MsgType {
  pub fn id(self) -> MsgTypeID {
    match self {
      Self::Ping => 1,
      Self::Error => 2,
      Self::FileReq => 3,
      Self::FileRes => 4,
      Self::PieceUploadReq => 5,
      Self::PieceUploadRes => 6,
    }
  }

  // NOTE: return Result<Self> instead of Option<Self>
  pub fn from_id(id: MsgTypeID) -> Option<Self> {
    Some(match id {
      1 => Self::Ping,
      2 => Self::Error,
      3 => Self::FileReq,
      4 => Self::FileRes,
      5 => Self::PieceUploadReq,
      6 => Self::PieceUploadRes,
      _ => return None,
    })
  }
}

#[derive(Debug, Clone)]
pub struct MessageMetadata {
  msg_type: MsgType,
  payload_len: MsgPayloadLen,
  // pub from: data::Peer,
  // pub to: data:: Peer,
}

#[allow(dead_code)]
impl MessageMetadata {
  pub fn new(msg_type: MsgType, payload_len: MsgPayloadLen) -> Self {
    Self { msg_type, payload_len }
  }

  pub fn serialize(self) -> Vec<u8> {
    let mut buf = self.msg_type().id().to_be_bytes().to_vec();
    buf.extend(self.payload_len().to_be_bytes().iter());
    buf
  }

  pub fn serialize_with_prefix(self, mut buf: Vec<u8>) -> Vec<u8> {
    buf.extend(self.serialize().iter());
    buf
  }

  pub fn deserialize_from_reader<R>(mut reader: R) -> Result<(Self, R), (io::Error, R)>
  where
    R: io::Read + io::Write,
  {
    Self::from_reader(reader)
  }

  pub fn from_reader<R>(mut reader: R) -> Result<(Self, R), (io::Error, R)>
  where
    R: io::Read + io::Write,
  {
    let mut buf = [0u8; mem::size_of::<MsgTypeID>()];
    if let Err(err) = reader.read(&mut buf) {
      return Err((err, reader));
    }
    let msg_type_id = MsgTypeID::from_be_bytes(buf);

    let mut buf = [0u8; mem::size_of::<MsgPayloadLen>()];
    if let Err(err) = reader.read(&mut buf) {
      return Err((err, reader));
    }
    let payload_len = MsgPayloadLen::from_be_bytes(buf);

    Ok((
      Self::new(
        match MsgType::from_id(msg_type_id) {
          Some(t) => t,
          None => return Err((io::Error::new(io::ErrorKind::Other, "invalid message-type-id"), reader)),
        },
        payload_len,
      ),
      reader,
    ))
  }

  pub fn msg_type(&self) -> MsgType {
    self.msg_type.clone()
  }

  pub fn payload_len(&self) -> MsgPayloadLen {
    self.payload_len
  }
}
