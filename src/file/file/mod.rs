#[allow(unused_imports)]
use std::{fs, io};
use uuid;

#[allow(unused_imports)]
use crate::file::chunk::{v1::Chunk, DEFAULT_CHUNK_SIZE};

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
  pub fn new(length: usize, title: Option<String>, created_at: Option<u64>, uploaded_at: Option<u64>) -> Self {
    Self {
      length,
      title,
      created_at,
      uploaded_at,
      ..Default::default()
    }
    .with_new_id()
  }

  pub fn new_with_id(
    id: FileID,
    length: usize,
    title: Option<String>,
    created_at: Option<u64>,
    uploaded_at: Option<u64>,
  ) -> Self {
    Self {
      length,
      title,
      created_at,
      uploaded_at,
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

  pub fn chunks(&self) -> Vec<Chunk> {
    (0..self.length)
      .step_by(DEFAULT_CHUNK_SIZE)
      .map(|i| {
        let len = if (self.length - (i * DEFAULT_CHUNK_SIZE)) > DEFAULT_CHUNK_SIZE {
          self.length - i * DEFAULT_CHUNK_SIZE
        } else {
          DEFAULT_CHUNK_SIZE
        };

        Chunk::new(self.id, len, i)
      })
      .collect()
  }
}
