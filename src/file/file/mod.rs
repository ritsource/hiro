#[allow(unused_imports)]
use std::{fs, io};
use uuid;

#[allow(unused_imports)]
use crate::file::piece::{v1::Piece, DEFAULT_PIECE_SIZE};

pub type FileID = uuid::Bytes;

#[derive(Default, Debug)]
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
    self.id = *uuid::Uuid::new_v4().as_bytes();
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
      .map(|i| {
        let len = if (self.length - (i * DEFAULT_PIECE_SIZE)) > DEFAULT_PIECE_SIZE {
          self.length - i * DEFAULT_PIECE_SIZE
        } else {
          DEFAULT_PIECE_SIZE
        };

        Piece::new(self.id, len, i)
      })
      .collect()
  }
}
