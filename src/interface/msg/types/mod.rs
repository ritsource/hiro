use serde_json;

#[allow(unused_imports)]
use std::any;
use std::io;
#[allow(unused_imports)]
use std::mem;

use super::data;

pub struct PiecesAndPeersForFileRequest(data::File);

pub const MAX_MEMORIZABLE_PAYLOAD_SIZE: usize = 1024 * 1024 * 10; // 10mb

impl PiecesAndPeersForFileRequest {
  pub fn new(data: data::File) -> Self {
    Self(data)
  }

  pub fn data(self) -> data::File {
    self.0
  }

  pub fn from_reader<R>(mut reader: R, offset: usize) -> Result<(Self, R), (io::Error, R)>
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

    match serde_json::from_slice::<data::File>(&payload[..]) {
      Ok(file) => Ok((Self::new(file), reader)),
      Err(err) => {
        println!("Error: {}", err);
        Err((io::Error::new(io::ErrorKind::Other, err), reader))
      }
    }
  }

  // pub fn from_vec(self, vec: Vec<u8>) -> Result((), io::Error) {}

  pub fn as_vec(self) -> Result<Vec<u8>, io::Error> {
    match serde_json::to_vec(&self.0) {
      Ok(vec) => Ok(vec),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
  }
}

pub struct PiecesAndPeersForFileResponse(Vec<(data::Piece, data::Peer)>);

impl PiecesAndPeersForFileResponse {
  pub fn new(data: Vec<(data::Piece, data::Peer)>) -> Self {
    Self(data)
  }

  pub fn data(self) -> Vec<(data::Piece, data::Peer)> {
    self.0
  }

  // copied for now
  pub fn from_reader<R>(mut reader: R, offset: usize) -> Result<(Self, R), (io::Error, R)>
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

    match serde_json::from_slice::<Vec<(data::Piece, data::Peer)>>(&payload[..]) {
      Ok(data) => Ok((Self::new(data), reader)),
      Err(err) => {
        println!("Error: {}", err);
        Err((io::Error::new(io::ErrorKind::Other, err), reader))
      }
    }
  }

  pub fn as_vec(self) -> Result<Vec<u8>, io::Error> {
    match serde_json::to_vec(&self.0) {
      Ok(vec) => Ok(vec),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
    }
  }
}
