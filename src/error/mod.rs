use std::error;
use std::fmt;
use std::io;

// pub enum ErrorKind {
//   Other,
// }

pub struct Error {
  // kind: ErrorKind,
  msg: String,
}

impl Error {
  pub fn new<M: Into<String>>(msg: M) -> Self {
    Error { msg: msg.into() }
  }

  pub fn into_message(self) -> String {
    self.msg
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "neo::Error: {}", self.msg)
  }
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "neo::Error: {}", self.msg)
  }
}

impl error::Error for Error {
  fn description(&self) -> &str {
    self.msg.as_str()
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Error::new(err.to_string())
  }
}
