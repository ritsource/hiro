use crate::file::{self, piece::v1 as piece};
use serde::{Deserialize, Serialize};

// NOTE: For copy of data - for every chunk there will
// be a primary worker and multiple secondary worker.

// 1. Chunks are calculated and initialized by master
// 2. Then it's sent to the client where it will write
// the data to the corresponding workers
// 3. Worker will handle the data given by client

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Piece {
  pub id: piece::PieceID,
  pub file_id: file::FileID,
  pub index: usize,
  pub length: usize,
  // pub data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Piece {}

impl From<piece::Piece> for Piece {
  fn from(p: piece::Piece) -> Self {
    Self {
      id: p.id,
      file_id: p.file_id,
      index: p.index,
      length: p.length,
    }
  }
}

impl Into<piece::Piece> for Piece {
  fn into(self) -> piece::Piece {
    piece::Piece {
      id: self.id,
      file_id: self.file_id,
      index: self.index,
      length: self.length,
    }
  }
}
