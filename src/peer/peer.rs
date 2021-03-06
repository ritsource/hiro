use serde::{Deserialize, Serialize};
use std::cell;
use std::io;
use std::net;

use std::io::Write;

use crate::id::v1 as id;
use crate::interface::message;
use crate::interface::payload;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PeerType {
  Master,
  Worker,
  Client,
}

pub type PeerID = id::ID;

#[derive(Debug)]
pub enum PeerState {
  BeingWritten,
  BeingRead,
  Free,
}

impl Default for PeerState {
  fn default() -> Self {
    Self::Free
  }
}

#[derive(Debug)]
pub struct PeerNew<'r> {
  pub id: PeerID,
  pub peer_type: PeerType,
  pub addr: net::SocketAddr,
  pub stream: cell::RefMut<'r, Option<net::TcpStream>>,
  pub state: PeerState,
}

// impl From<peer::PeerNew> for Peer {
//   fn from(p: peer::PeerNew) -> Self {
//     Self {
//       id: p.id,
//       peer_type: p.peer_type,
//       addr: p.addr,
//     }
//   }
// }
//
// impl Into<peer::PeerNew> for Peer {
//   fn into(self) -> peer::PeerNew {
//     peer::Peer {
//       id: self.id,
//       peer_type: self.peer_type,
//       addr: self.addr,
//       stream: Default::default(),
//       state: Default::default(),
//     }
//   }
// }

#[derive(Debug)]
pub struct Peer {
  pub id: PeerID,
  pub peer_type: PeerType,
  pub addr: net::SocketAddr,
  pub stream: Option<net::TcpStream>,
  pub state: PeerState,
}

#[allow(dead_code)]
impl Peer {
  pub fn new(addr: net::SocketAddr, peer_type: PeerType) -> Self {
    Self {
      id: PeerID::new(),
      peer_type: peer_type,
      addr: addr,
      stream: None,
      state: PeerState::Free,
    }
  }

  pub fn new_master(addr: &str) -> Result<Self, std::net::AddrParseError> {
    Ok(Self::new(
      match addr.parse() {
        Ok(a) => a,
        Err(e) => {
          return Err(e);
        }
      },
      PeerType::Master,
    ))
  }

  pub fn new_worker(addr: &str) -> Result<Self, std::net::AddrParseError> {
    Ok(Self::new(
      match addr.parse() {
        Ok(a) => a,
        Err(e) => {
          return Err(e);
        }
      },
      PeerType::Worker,
    ))
  }

  pub fn new_client(addr: &str) -> Result<Self, std::net::AddrParseError> {
    Ok(Self::new(
      match addr.parse() {
        Ok(a) => a,
        Err(e) => {
          return Err(e);
        }
      },
      PeerType::Client,
    ))
  }

  pub fn with_id(mut self, id: PeerID) -> Self {
    self.id = id;
    self
  }

  pub fn peer_type(self) -> PeerType {
    self.peer_type
  }

  pub fn peer_id(self) -> PeerID {
    self.id
  }

  pub fn id(self) -> PeerID {
    self.id
  }

  pub fn connection(self) -> Option<net::TcpStream> {
    self.stream
  }

  pub fn is_connected(&self) -> bool {
    if let None = self.stream {
      false
    } else {
      true
    }
  }

  pub fn connect(&mut self) -> Result<(), io::Error> {
    match net::TcpStream::connect(&self.addr) {
      Ok(stream) => {
        self.stream = Some(stream);
        Ok(())
      }
      Err(err) => Err(err),
    }
  }

  pub async fn write_message<'de, P, D>(
    &mut self,
    msg_type: message::MsgType,
    pld: P,
  ) -> Result<usize, io::Error>
  where
    P: payload::Payload<'de, D>,
    D: serde::Serialize + serde::Deserialize<'de>,
  {
    if !self.is_connected() {
      self.connect()?;
    }
    let msg_buf = message::message_buffer_from_payload(msg_type, pld)?;

    self.write(&msg_buf)
  }
}

impl io::Read for Peer {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
    match &mut self.stream {
      Some(s) => s.read(buf),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "connection does not exist",
      )),
    }
  }
}

impl io::Write for Peer {
  fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
    match &mut self.stream {
      Some(s) => s.write(buf),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "connection does not exist",
      )),
    }
  }

  fn flush(&mut self) -> Result<(), io::Error> {
    match &mut self.stream {
      Some(s) => s.flush(),
      None => Err(io::Error::new(
        io::ErrorKind::Other,
        "connection does not exist",
      )),
    }
  }
}
