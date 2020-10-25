use crate::file::{self, piece::v1 as piece};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PieceWithData {
  pub id: piece::PieceID,
  pub file_id: file::FileID,
  pub start: u32,
  pub length: u32,
  data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl PieceWithData {
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

impl From<piece::Piece> for PieceWithData {
  fn from(p: piece::Piece) -> Self {
    Self {
      id: p.id(),
      file_id: p.file_id(),
      start: p.start(),
      length: p.length(),
      data: p.data(),
    }
  }
}

impl Into<piece::Piece> for PieceWithData {
  fn into(self) -> piece::Piece {
    // since length and start both arguements are of same type, to avoid any
    // error we are making sure that we pass the correct values to Piece
    // using specific methods for start (with_start) and length (with_length)
    piece::Piece::new_with_id(self.id, self.file_id, self.start, self.length)
      .with_start(self.start)
      .with_length(self.length)
      .with_data(self.data())
  }
}

impl From<super::piece::Piece> for PieceWithData {
  fn from(p: super::piece::Piece) -> Self {
    Self {
      id: p.id,
      file_id: p.file_id,
      start: p.start,
      length: p.length,
      data: None,
    }
  }
}

impl Into<super::piece::Piece> for PieceWithData {
  fn into(self) -> super::piece::Piece {
    super::piece::Piece {
      id: self.id,
      file_id: self.file_id,
      start: self.start,
      length: self.length,
    }
  }
}
