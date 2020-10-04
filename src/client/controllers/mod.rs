use std::fs;
use std::io;
use std::path;

use std::io::prelude::{Read, Seek, Write};

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

pub async fn read_file_content(path: &path::Path, start: usize, length: usize) -> Result<Vec<u8>, io::Error> {
  let mut f = fs::File::open("data/ipsum.text")?;
  f.seek(io::SeekFrom::Start(start as u64))?;

  let mut total: usize = 0;
  let mut buf: Vec<u8> = vec![];
  let mut b = [0u8; 64];

  while match f.read(&mut b) {
    Ok(nr) => {
      total += nr;
      buf.append(&mut b[..nr].to_vec());

      if total >= length {
        false
      } else if nr == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "EOF"));
      } else {
        true
      }
    }
    Err(err) => return Err(err),
  } {}

  Ok(buf)
}
