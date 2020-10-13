use std::io;

use super::super::payload;
use crate::constants;

pub fn gen_buf_for_rpc(msg: super::MsgType, payload: Vec<u8>) -> Vec<u8> {
  let mut buf = constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(msg.id().to_be_bytes().iter());

  buf.extend(((payload.len()) as super::MsgPayloadLen).to_be_bytes().iter());
  buf.extend(payload);
  buf
}

pub fn new_message_buffer<'de, P, D>(msg: super::MsgType, pld: P) -> Result<Vec<u8>, io::Error>
where
  P: payload::Payload<'de, D>,
  D: serde::Serialize + serde::Deserialize<'de>,
{
  let pld_buf = pld.as_vec()?;

  let mut buf = constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(msg.id().to_be_bytes().iter());

  buf.extend(((pld_buf.len()) as super::MsgPayloadLen).to_be_bytes().iter());
  buf.extend(pld_buf);

  Ok(buf)
}
