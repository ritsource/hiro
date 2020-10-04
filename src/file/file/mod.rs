use crate::constants::DEFAULT_PIECE_SIZE;
#[allow(unused_imports)]
use crate::file::piece::v1::Piece;
use crate::id::v1 as id;

pub type FileID = id::ID;

#[derive(Default, Debug, Clone)]
pub struct File {
  pub id: FileID,
  pub length: usize,
  pub title: Option<String>,
  pub created_at: Option<u64>,
  pub uploaded_at: Option<u64>,
}

#[allow(dead_code)]
impl File {
  pub fn new(length: usize, title: Option<String>) -> Self {
    Self {
      length,
      title,
      ..Default::default()
    }
    .with_new_id()
  }

  pub fn new_with_id(id: FileID, length: usize, title: Option<String>) -> Self {
    Self {
      length,
      title,
      ..Default::default()
    }
    .with_id(id)
  }

  fn with_new_id(mut self) -> Self {
    self.id = FileID::new();
    self
  }

  pub fn with_id(mut self, id: FileID) -> Self {
    self.id = id;
    self
  }

  pub fn with_len(mut self, length: usize) -> Self {
    self.length = length;
    self
  }

  pub fn id(self) -> FileID {
    self.id
  }

  pub fn len(self) -> usize {
    self.length
  }

  pub fn pieces(&self) -> Vec<Piece> {
    (0..self.length)
      .step_by(DEFAULT_PIECE_SIZE)
      .map(|start| {
        let len = if (self.length - start) > DEFAULT_PIECE_SIZE {
          DEFAULT_PIECE_SIZE
        } else {
          self.length - start
        };

        Piece::new(self.id, start as u32, len as u32)
      })
      .collect()
  }
}
