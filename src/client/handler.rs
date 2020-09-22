use std::io;
use std::net;

use io::Write;

use crate::constants;
use crate::interface::message;
use crate::interface::message::payload::Payload;

pub fn handle_stream(stream: net::TcpStream) -> Result<net::TcpStream, (io::Error, net::TcpStream)> {
  match message::Header::from_reader(stream) {
    Ok((header, mut stream)) => {
      if header.proto_id() != constants::PROTOCOL_IDENTIFIER_V1 {
        if let Err(err) = stream.write(&message::gen_buf_for_rpc(
          message::Message::Error,
          b"invalid protocol identifier".to_vec(),
        )) {
          return Err((err, stream));
        }
        return Ok(stream);
      }

      match message::Message::from_id(header.message_id()) {
        Some(message::Message::Ping) => {
          println!("message recieved: a ping");
          Ok(stream)
        }
        Some(message::Message::FileRes) => {
          match message::payload::FileRes::from_reader(stream, header.payload_len() as usize) {
            Ok((payload, stream)) => {
              println!("{:?}", payload.data());
              Ok(stream)
            }
            Err((err, stream)) => Err((err, stream)),
          }
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
