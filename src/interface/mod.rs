pub mod data;
pub mod message;

use std::io;
use std::mem;
use std::net;

use std::io::prelude::{Read, Write};

use crate::constants;
use crate::master;
use message::payload::Payload;

pub fn handle_stream(mut stream: net::TcpStream) -> Result<net::TcpStream, (io::Error, net::TcpStream)> {
  use constants::PROTOCOL_IDENTIFIER_V1;

  let mut buf = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
  if let Err(err) = stream.read(&mut buf) {
    return Err((err, stream));
  }
  let mut pid = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
  pid.copy_from_slice(&buf);

  if pid != PROTOCOL_IDENTIFIER_V1 {
    println!("invalid message recieved, protocol identifier not valid");
    // NOTE: create a error-message (Message), and
    // write the error-message as response
    if let Err(err) = stream.write(b"invalid protocol id") {
      return Err((err, stream));
    }
    return Ok(stream);
  }

  let mut buf = [0u8; mem::size_of::<message::MessageID>()];
  if let Err(err) = stream.read(&mut buf) {
    return Err((err, stream));
  }
  let message_type_id = message::MessageID::from_be_bytes(buf);

  println!("successfully recieved message");

  let mut buf = [0u8; mem::size_of::<message::MessagePayloadLength>()];
  if let Err(err) = stream.read(&mut buf) {
    return Err((err, stream));
  }
  let payload_len = message::MessagePayloadLength::from_be_bytes(buf);

  match message::Message::from_id(message_type_id) {
    Some(message::Message::FileReq) => {
      match message::payload::FileReq::from_reader(stream, payload_len as usize) {
        Ok((req_payload, mut stream)) => {
          let res_payload = master::controllers::calculate_pieces(req_payload).as_vec().unwrap();

          if let Err(err) = stream.write(&message::build_message_buffer(message::Message::FileRes, res_payload)) {
            return Err((err, stream));
          }
          return Ok(stream);
        }
        Err((err, stream)) => {
          println!("an error occurred, coudn't parse message payload, {}", err);
          Err((err, stream))
          // match stream.write(&buf) {
          //   Ok(nw) => {
          //     println!("successfully written {} bytes", nw);
          //   }
          //   Err(err) => {
          //     println!("an error occurred, couldn't write to connection: {}", err);
          //   }
          // };
        }
      }
    }
    Some(message::Message::FileRes) => {
      println!("message recieved, response for file message");

      match message::payload::FileRes::from_reader(stream, payload_len as usize) {
        Ok((payload, stream)) => {
          println!("{:?}", payload.data());
          Ok(stream)
        }
        Err((err, stream)) => {
          // println!("an error occurred, coudn't parse message payload, {}", err);
          Err((err, stream))
        }
      }
    }
    Some(message::Message::Ping) => {
      println!("message recieved, a ping from {}", stream.peer_addr().unwrap());
      Ok(stream)
    }
    _ => Ok(stream),
  }
}
