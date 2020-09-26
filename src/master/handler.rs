use std::io;
use std::net;

use io::{Read, Write};

use super::controllers;
use crate::constants;
use crate::interface::message;
use crate::interface::payload;
use crate::interface::payload::Payload;

pub fn handle_stream(mut stream: net::TcpStream) -> Result<net::TcpStream, (io::Error, net::TcpStream)> {
  use constants::PROTOCOL_IDENTIFIER_V1;

  let mut buf = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
  if let Err(err) = stream.read(&mut buf) {
    return Err((err, stream));
  }
  let mut proto_id = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
  proto_id.copy_from_slice(&buf);

  if proto_id != constants::PROTOCOL_IDENTIFIER_V1 {
    if let Err(err) = stream.write(&message::gen_buf_for_rpc(
      message::MsgType::Error,
      b"invalid protocol identifier".to_vec(),
    )) {
      return Err((err, stream));
    }
    return Ok(stream);
  }

  match message::Message::from_reader(stream) {
    Ok((msg, stream)) => {
      match msg.msg_type() {
        message::MsgType::Ping => {
          println!("message recieved: a ping");
          Ok(stream)
        }
        message::MsgType::FileReq => match payload::FileReq::from_reader(stream, msg.payload_len() as usize) {
          Ok((req_payload, mut stream)) => {
            let res_payload = controllers::calculate_pieces(req_payload).as_vec().unwrap();

            if let Err(err) = stream.write(&message::gen_buf_for_rpc(message::MsgType::FileRes, res_payload)) {
              return Err((err, stream));
            }
            return Ok(stream);
          }
          Err((err, mut stream)) => {
            match stream.write(&message::gen_buf_for_rpc(
              message::MsgType::Error,
              format!("couldn't parse payload, {}", err).into_bytes(),
            )) {
              Ok(_nw) => Ok(stream),
              Err(err) => Err((err, stream)),
            }
          }
        },
        // NOTE: check for MESSAGE-TYPE-ID-INVALID separately
        _ => {
          println!("message recieved: unknown");
          // NOTE: maybe respond with a error message
          Ok(stream)
        }
      }
    } // NOTE: treat io::Error::EOF separately
    Err((err, stream)) => Err((err, stream)),
  }
}
