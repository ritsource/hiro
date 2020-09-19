pub mod types;

use std::mem;
use std::net;

use std::io::prelude::*;

use super::data;

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
  let mut buf = [0u8; mem::size_of::<MessageTypeID>()];
  match stream.read(&mut buf) {
    Ok(nr) => println!("- read {} bytes", nr),
    Err(err) => {
      println!("an error occurred, {}", err);
      println!("terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(net::Shutdown::Both).unwrap();
    }
  };

  println!("successfully recieved message");

  match message_type_from_message_type_id(MessageTypeID::from_be_bytes(buf)) {
    MessageType::PiecesAndPeersForFileRequest => match types::PiecesAndPeersForFileRequest::from_reader(stream) {
      Ok(x) => {
        println!("{:?}", x.data());
      }
      Err(err) => {
        println!("an error occurred, {}", err);
      }
    },
    MessageType::PiecesAndPeersForFileResponse => {}
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
