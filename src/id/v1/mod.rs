use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::hash::Hash;
use uuid;

#[derive(Serialize, Deserialize, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ID(uuid::Bytes);

impl ID {
  pub fn new() -> Self {
    Self(*uuid::Uuid::new_v4().as_bytes())
  }

  pub fn from_bytes(bytes: uuid::Bytes) -> Self {
    Self(bytes)
  }

  pub fn as_bytes(self) -> uuid::Bytes {
    self.0
  }

  pub fn to_hex_string(&self) -> String {
    self
      .0
      .to_vec()
      .iter_mut()
      .map(|b| format!("{:x}", b))
      .collect::<Vec<String>>()
      .join("")
  }
}

impl fmt::Display for ID {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_hex_string())
  }
}

impl fmt::Debug for ID {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.to_hex_string())
  }
}
