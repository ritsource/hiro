use serde::{Deserialize, Serialize};
use std::io;
use std::net::{SocketAddr, TcpStream};

use crate::id::v1 as id;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PeerType {
  Master,
  Worker,
  Client,
}

pub type PeerID = id::ID;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Peer {
  pub id: PeerID,
  pub peer_type: PeerType,
  pub addr: SocketAddr,
  pub stream: Option<TcpStream>,
}

#[allow(dead_code)]
impl Peer {
  pub fn new(addr: SocketAddr, peer_type: PeerType) -> Self {
    Self {
      id: PeerID::new(),
      peer_type: peer_type,
      addr: addr,
      stream: None,
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

  pub fn connection(self) -> Option<TcpStream> {
    self.stream
  }

  pub fn connect(&mut self) -> Result<(), String> {
    match TcpStream::connect(&self.addr) {
      Ok(s) => {
        self.stream = Some(s);
        Ok(())
      }
      Err(e) => Err(format!("{:?}", e)),
    }
  }
}

impl io::Read for Peer {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
    match &mut self.stream {
      Some(s) => s.read(buf),
      None => Err(io::Error::new(io::ErrorKind::Other, "connection does not exist")),
    }
  }
}

impl io::Write for Peer {
  fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
    match &mut self.stream {
      Some(s) => s.write(buf),
      None => Err(io::Error::new(io::ErrorKind::Other, "connection does not exist")),
    }
  }

  fn flush(&mut self) -> Result<(), io::Error> {
    match &mut self.stream {
      Some(s) => s.flush(),
      None => Err(io::Error::new(io::ErrorKind::Other, "connection does not exist")),
    }
  }
}
