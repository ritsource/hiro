use serde::{Deserialize, Serialize};
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerType {
  Master,
  Worker,
  Client,
}

pub type PeerID = uuid::Bytes;

#[allow(dead_code)]
pub struct Peer<A: ToSocketAddrs> {
  pub id: PeerID,
  pub peer_type: PeerType,
  pub addr: A,
  pub stream: Option<TcpStream>,
}

#[allow(dead_code)]
impl<A> Peer<A>
where
  A: ToSocketAddrs,
{
  pub fn new(addr: A, peer_type: PeerType) -> Self {
    Self {
      id: *uuid::Uuid::new_v4().as_bytes(),
      peer_type: peer_type,
      addr: addr,
      stream: None,
    }
  }

  pub fn new_master(addr: A) -> Self {
    Self::new(addr, PeerType::Master)
  }

  pub fn new_worker(addr: A) -> Self {
    Self::new(addr, PeerType::Worker)
  }

  pub fn new_client(addr: A) -> Self {
    Self::new(addr, PeerType::Client)
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

impl<A> io::Read for Peer<A>
where
  A: ToSocketAddrs,
{
  fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
    match &mut self.stream {
      Some(s) => s.read(buf),
      None => Err(io::Error::new(io::ErrorKind::Other, "connection does not exist")),
    }
  }
}

impl<A> io::Write for Peer<A>
where
  A: ToSocketAddrs,
{
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
