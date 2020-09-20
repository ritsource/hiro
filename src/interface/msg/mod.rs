pub mod types;

use std::mem;
use std::net;

use std::io::prelude::*;

use crate::master;

use super::data;

pub type MessagePayloadLength = u32;

pub type MessageTypeID = u16;

pub enum MessageType {
  PiecesAndPeersForFileRequest,
  PiecesAndPeersForFileResponse,
  Ping,
}

impl Default for MessageType {
  fn default() -> Self {
    MessageType::Ping
  }
}

pub fn handle_stream(mut stream: net::TcpStream) {
  use super::constants::PROTOCOL_IDENTIFIER_V1;

  let mut buf = [0u8; PROTOCOL_IDENTIFIER_V1.len()];

  match stream.read(&mut buf) {
    Ok(nr) => {
      println!("- read {} bytes", nr);

      let mut pid = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
      pid.copy_from_slice(&buf[0..PROTOCOL_IDENTIFIER_V1.len()]);

      if pid == PROTOCOL_IDENTIFIER_V1 {
        handle_stream_for_message(stream);
      } else {
        println!("an error occurred, invalid protocol identifier");
      }
    }
    Err(err) => {
      println!("an error occurred, {}", err);
      println!("terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(net::Shutdown::Both).unwrap();
    }
  }
}

pub fn handle_stream_for_message(mut stream: net::TcpStream) {
  let mut buf = [0u8; mem::size_of::<MessageTypeID>()];
  match stream.read(&mut buf) {
    Ok(nr) => println!("- read {} bytes", nr),
    Err(err) => {
      println!("an error occurred, {}", err);
      println!("terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(net::Shutdown::Both).unwrap();
    }
  };
  let msg_type_id = MessageTypeID::from_be_bytes(buf);

  println!("successfully recieved message");

  let mut buf = [0u8; mem::size_of::<MessagePayloadLength>()];
  match stream.read(&mut buf) {
    Ok(nr) => println!("- read {} bytes", nr),
    Err(err) => {
      println!("an error occurred, {}", err);
      println!("terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(net::Shutdown::Both).unwrap();
    }
  };
  let payload_len = MessagePayloadLength::from_be_bytes(buf);

  match message_type_from_message_type_id(msg_type_id) {
    MessageType::PiecesAndPeersForFileRequest => {
      match types::PiecesAndPeersForFileRequest::from_reader(stream, payload_len as usize) {
        Ok((payload, mut stream)) => {
          let resp_payload = master::controllers::calculate_pieces(payload).as_vec().unwrap();

          let mut resp_buf = super::constants::PROTOCOL_IDENTIFIER_V1.to_vec();
          resp_buf.extend(
            message_type_id_from_message_type(MessageType::PiecesAndPeersForFileResponse)
              .to_be_bytes()
              .iter(),
          );
          resp_buf.extend(((resp_payload.len()) as MessagePayloadLength).to_be_bytes().iter());
          resp_buf.extend(resp_payload);

          match stream.write(&resp_buf) {
            Ok(nw) => {
              println!("successfully written {} bytes", nw);
            }
            Err(err) => {
              println!("an error occurred, couldn't write to connection: {}", err);
            }
          };
        }
        Err((err, _stream)) => {
          println!("an error occurred, coudn't parse message payload, {}", err);
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
    MessageType::PiecesAndPeersForFileResponse => {
      println!("message recieved, response for file message");

      match types::PiecesAndPeersForFileResponse::from_reader(stream, payload_len as usize) {
        Ok((payload, _stream)) => {
          println!("{:?}", payload.data());
        }
        Err((err, _stream)) => {
          println!("an error occurred, coudn't parse message payload, {}", err);
        }
      }
    }
    MessageType::Ping => println!("message recieved, a ping from {}", stream.peer_addr().unwrap()),
  }
}

pub fn message_type_from_message_type_id(message_type_id: MessageTypeID) -> MessageType {
  match message_type_id {
    1 => MessageType::PiecesAndPeersForFileRequest,
    2 => MessageType::PiecesAndPeersForFileResponse,
    _ => MessageType::Ping,
  }
}

pub fn message_type_id_from_message_type(message_type: MessageType) -> MessageTypeID {
  match message_type {
    MessageType::PiecesAndPeersForFileRequest => 1,
    MessageType::PiecesAndPeersForFileResponse => 2,
    MessageType::Ping => 0,
  }
}
