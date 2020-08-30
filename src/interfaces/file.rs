use crate::file;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct File {
  pub id: file::FileID,
  pub length: usize,
  pub title: Option<String>,
  pub created_at: Option<u64>,
  pub uploaded_at: Option<u64>,
}

impl From<file::File> for File {
  fn from(f: file::File) -> Self {
    Self {
      id: f.id,
      length: f.length,
      title: f.title,
      created_at: f.created_at,
      uploaded_at: f.uploaded_at,
      ..Default::default()
    }
  }
}

impl Into<file::File> for File {
  fn into(self) -> file::File {
    file::File {
      id: self.id,
      length: self.length,
      title: self.title,
      created_at: self.created_at,
      uploaded_at: self.uploaded_at,
      ..Default::default()
    }
  }
}
