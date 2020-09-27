use std::io;
use std::net;

use io::{Read, Write};

use crate::constants;
use crate::interface::message;
use crate::interface::payload;
use crate::interface::payload::Payload;

use super::controllers;

#[allow(dead_code)]
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
        message::MsgType::PieceUploadReq => {
          match payload::PieceUploadReq::from_reader(stream, msg.payload_len() as usize) {
            Ok((pld, mut stream)) => {
              println!("message recieved: piece upload");
              println!("{:?}", pld.clone().data());

              let resp_data = match controllers::handle_piece_upload_message(pld) {
                Ok(pld) => message::gen_buf_for_rpc(message::MsgType::PieceUploadRes, pld.as_vec().unwrap()),
                Err(err) => message::gen_buf_for_rpc(message::MsgType::Error, format!("{}", err).as_bytes().to_vec()),
              };

              match stream.write(&resp_data) {
                Ok(nw) => {
                  println!("written {} bytes", nw);
                  Ok(stream)
                }
                Err(err) => Err((err, stream)),
              }
            }
            Err((err, stream)) => Err((err, stream)),
          }
        }
        message::MsgType::Error => {
          println!("message recieved: error");
          Err((io::Error::new(io::ErrorKind::Other, "error message recieved"), stream))
        }
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
