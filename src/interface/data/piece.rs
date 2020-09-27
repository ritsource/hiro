use crate::file::{self, piece::v1 as piece};
use serde::{Deserialize, Serialize};

// NOTE: For copy of data - for every chunk there will
// be a primary worker and multiple secondary worker.

// 1. Chunks are calculated and initialized by master
// 2. Then it's sent to the client where it will write
// the data to the corresponding workers
// 3. Worker will handle the data given by client

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Piece {
  pub id: piece::PieceID,
  pub file_id: file::FileID,
  pub index: usize,
  pub length: usize,
  data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Piece {
  pub fn with_data(mut self, data: Option<Vec<u8>>) -> Self {
    self.data = data;
    self
  }

  pub fn data(self) -> Option<Vec<u8>> {
    self.data
  }

  pub fn has_data(self) -> bool {
    self.data != None
  }
}

impl From<piece::Piece> for Piece {
  fn from(p: piece::Piece) -> Self {
    Self {
      id: p.id,
      file_id: p.file_id,
      index: p.index,
      length: p.length,
      data: p.data(),
    }
  }
}

impl Into<piece::Piece> for Piece {
  fn into(self) -> piece::Piece {
    piece::Piece::new_with_id(self.id, self.file_id, self.length, self.index).with_data(self.data())
  }
}
