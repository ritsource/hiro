pub mod payload;

use crate::constants;

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

pub fn build_message_buffer(msg: Message, payload: Vec<u8>) -> Vec<u8> {
  let mut buf = constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(msg.id().to_be_bytes().iter());
  buf.extend(((payload.len()) as MessagePayloadLength).to_be_bytes().iter());
  buf.extend(payload);
  buf
}
