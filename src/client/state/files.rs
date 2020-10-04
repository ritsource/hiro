use std::collections::HashMap;
use std::io;
use tokio::sync::Mutex;

use crate::file;

#[derive(Clone)]
pub struct FileInfo {
  path: String,
  // upload_done: bool,
}

impl FileInfo {
  pub fn new<P: Into<String>>(path: P) -> Self {
    Self { path: path.into() }
  }

  pub fn path(self) -> String {
    self.path
  }
}

lazy_static! {
  static ref FILES: Mutex<HashMap<file::FileID, FileInfo>> = Mutex::new(HashMap::new());
}

pub async fn get_file(id: &file::FileID) -> Option<FileInfo> {
  let files = FILES.lock().await;

  if let Some(info) = files.get(id) {
    Some(info.clone())
  } else {
    None
  }
}

pub async fn add_file(id: &file::FileID, path: &str) -> Result<(), io::Error> {
  let mut files = FILES.lock().await;

  if let None = files.get(id) {
    let _ = files.insert(*id, FileInfo::new(path));
    Ok(())
  } else {
    Err(io::Error::new(io::ErrorKind::Other, "File with id already exists"))
  }
}

pub async fn update_file(id: &file::FileID, path: Option<String>) -> Result<(), io::Error> {
  let mut files = FILES.lock().await;

  match files.get(id) {
    Some(info) => {
      let info = info.clone();
      let path = match path {
        Some(p) => p,
        None => info.path(),
      };
      let _ = files.insert(*id, FileInfo::new(path));
      Ok(())
    }
    None => Err(io::Error::new(io::ErrorKind::Other, "File with id does not exist")),
  }
}

pub async fn remove_file(id: &file::FileID) {
  let mut files = FILES.lock().await;
  let _ = files.remove(id);
}
