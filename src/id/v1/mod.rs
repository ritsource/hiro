use serde::{Deserialize, Serialize};
use std::fmt;
use uuid;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
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

  pub fn as_hex_string(&self) -> String {
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
    write!(f, "{}", self.as_hex_string())
  }
}

impl fmt::Debug for ID {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_hex_string())
  }
}
