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
  pub start: u32,
  pub length: u32,
  // data: Option<Vec<u8>>,
}

impl From<piece::Piece> for Piece {
  fn from(p: piece::Piece) -> Self {
    Self {
      id: p.id(),
      file_id: p.file_id(),
      start: p.start(),
      length: p.length(),
    }
  }
}

impl Into<piece::Piece> for Piece {
  fn into(self) -> piece::Piece {
    // since length and start both arguements are of same type, to avoid any
    // error we are making sure that we pass the correct values to Piece
    // using specific methods for start (with_start) and length (with_length)
    piece::Piece::new_with_id(self.id, self.file_id, self.start, self.length)
      .with_start(self.start)
      .with_length(self.length)
  }
}
