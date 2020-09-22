use crate::constants;

pub fn gen_buf_for_rpc(msg: super::Message, payload: Vec<u8>) -> Vec<u8> {
  let mut buf = constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(msg.id().to_be_bytes().iter());

  buf.extend(((payload.len()) as super::MessagePayloadLength).to_be_bytes().iter());
  buf.extend(payload);
  buf
}
