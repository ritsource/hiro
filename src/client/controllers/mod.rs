use std::fs;
use std::io;
use std::path;

use crate::file;

pub fn read_file_matadata(path: &path::Path) -> Result<file::File, io::Error> {
  let metadata = fs::metadata(path)?;

  if !metadata.is_file() {
    return Err(io::Error::new(io::ErrorKind::InvalidInput, "not a file"));
  }

  Ok(file::File::new(
    metadata.len() as usize,
    path
      .file_name()
      .map_or(None, |v| v.to_str())
      .map_or(None, |v| Some(v.to_owned())),
  ))
}
