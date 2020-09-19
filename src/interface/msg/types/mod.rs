use serde_json;
#[allow(unused_imports)]
use std::any;
use std::io;
#[allow(unused_imports)]
use std::mem;

use super::data;

pub struct PiecesAndPeersForFileRequest(data::File);

impl PiecesAndPeersForFileRequest {
  pub fn new(data: data::File) -> Self {
    Self(data)
  }

  pub fn data(self) -> data::File {
    self.0
  }

  pub fn from_reader<R>(reader: R) -> Result<Self, io::Error>
  where
    R: io::Read,
  {
    // let mut buf = [0u8; mem::size_of::<data::File>()];

    // match reader.read(&mut buf) {
    //   Ok(nr) => {
    //     println!("- read {} bytes", nr);
    //     Ok(reader)
    //   }
    //   Err(err) => Err((err, reader)),
    // }
    match serde_json::from_reader(reader) {
      Ok(file) => Ok(Self::new(file)),
      Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
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
}
