use serde;
use serde_json;

use std::io;
use std::marker;

use super::data;

pub const MAX_MEMORIZABLE_PAYLOAD_SIZE: usize = 1024 * 1024 * 10; // 10mb

// NOTE: create serialize and deserialize traits
// and, make payload to be it's super trait
pub trait Payload<'de, D: serde::Serialize + serde::Deserialize<'de>>: marker::Sized {
  fn new(data: D) -> Self;

  fn data(self) -> D;

  fn as_vec(self) -> Result<Vec<u8>, io::Error>;

  fn from_vec(payload: Vec<u8>) -> Result<Self, io::Error>;

  fn from_reader<R>(mut reader: R, offset: usize) -> Result<(Self, R), (io::Error, R)>
  where
    R: io::Read,
  {
    if offset > MAX_MEMORIZABLE_PAYLOAD_SIZE {
      return Err((
        io::Error::new(io::ErrorKind::Other, "payload size exceeded maximum memorizable limit"),
        reader,
      ));
    }

    let mut total: usize = 0;
    // let mut payload: Vec<u8> = Vec::default();
    let mut payload: Vec<u8> = vec![];
    let mut buf = [0u8; 64];

    // Handle EOF separately
    while match reader.read(&mut buf) {
      Ok(nr) => {
        println!("- read {} bytes", nr);
        total += nr;
        payload.append(&mut buf[..nr].to_vec());

        if total >= offset {
          false
        } else if nr == 0 {
          // NOTE: need a better way of handling this,
          // maybe timeout or something
          return Err((io::Error::new(io::ErrorKind::Other, "unable to read any data"), reader));
        } else {
          true
        }
      }
      Err(err) => return Err((err, reader)),
    } {}

    match Self::from_vec(payload) {
      Ok(msg_payload) => Ok((msg_payload, reader)),
      Err(err) => Err((io::Error::new(io::ErrorKind::Other, err), reader)),
    }
  }
}

pub struct FileReq(data::File);

impl Payload<'_, data::File> for FileReq {
  fn new(data: data::File) -> Self {
    Self(data)
  }

  fn data(self) -> data::File {
    self.0
  }

  fn from_vec(payload: Vec<u8>) -> Result<Self, io::Error> {
    match serde_json::from_slice::<data::File>(&payload) {
      Ok(data) => Ok(Self::new(data)),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
  }

  fn as_vec(self) -> Result<Vec<u8>, io::Error> {
    match serde_json::to_vec(&self.0) {
      Ok(vec) => Ok(vec),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
  }
}

pub struct FileRes(Vec<(data::Piece, data::Peer)>);

impl Payload<'_, Vec<(data::Piece, data::Peer)>> for FileRes {
  fn new(data: Vec<(data::Piece, data::Peer)>) -> Self {
    Self(data)
  }

  fn data(self) -> Vec<(data::Piece, data::Peer)> {
    self.0
  }

  fn from_vec(payload: Vec<u8>) -> Result<Self, io::Error> {
    match serde_json::from_slice::<Vec<(data::Piece, data::Peer)>>(&payload) {
      Ok(data) => Ok(Self::new(data)),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
  }

  fn as_vec(self) -> Result<Vec<u8>, io::Error> {
    match serde_json::to_vec(&self.0) {
      Ok(vec) => Ok(vec),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
  }
}
