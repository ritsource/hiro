use std::collections::HashMap;
use std::io;
use tokio::sync::Mutex;

use crate::file;

#[derive(Clone)]
pub struct FileInfo {
  path: String,
  // upload_done: bool,
}

#[allow(dead_code)]
impl FileInfo {
  pub fn new<P: Into<String>>(p: P) -> Self {
    Self { path: p.into() }
  }

  fn with_path<P: Into<String>>(mut self, p: P) -> Self {
    self.path = p.into();
    self
  }

  pub fn path(self) -> String {
    self.path
  }
}

lazy_static! {
  static ref FILES: Mutex<HashMap<file::FileID, FileInfo>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
pub async fn get(id: &file::FileID) -> Option<FileInfo> {
  let files = FILES.lock().await;

  if let Some(info) = files.get(id) {
    Some(info.clone())
  } else {
    None
  }
}

#[allow(dead_code)]
pub async fn add(id: &file::FileID, path: &str) -> Result<(), io::Error> {
  let mut files = FILES.lock().await;

  if let None = files.get(id) {
    let _ = files.insert(*id, FileInfo::new(path));
    Ok(())
  } else {
    Err(io::Error::new(io::ErrorKind::Other, "File with id already exists"))
  }
}

#[allow(dead_code)]
pub async fn update(id: &file::FileID, path: Option<String>) -> Result<(), io::Error> {
  let mut files = FILES.lock().await;

  match files.get(id) {
    Some(info) => {
      let info = info.clone();
      let info = match path {
        Some(p) => info.with_path(p),
        None => info.clone(),
      };
      let _ = files.insert(*id, info);
      Ok(())
    }
    None => Err(io::Error::new(io::ErrorKind::Other, "File with id does not exist")),
  }
}

#[allow(dead_code)]
pub async fn remove(id: &file::FileID) {
  let mut files = FILES.lock().await;
  let _ = files.remove(id);
}
