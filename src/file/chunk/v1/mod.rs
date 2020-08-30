use std::alloc;
use std::mem;
use uuid;

use crate::file::chunk::VersionID;
use crate::file::file::FileID;

pub const VERSION: VersionID = [0u8; 3];
pub const HEADER_LEN: usize =
  mem::size_of::<VersionID>() + mem::size_of::<[u8; 4]>() + mem::size_of::<[u8; 4]>() + mem::size_of::<ChunkID>();

pub type Header = [u8; HEADER_LEN];

pub type ChunkID = uuid::Bytes;

#[derive(Default, Debug)]
pub struct Chunk {
  pub id: ChunkID,
  pub file_id: FileID,
  pub index: usize,
  pub length: usize,
}

#[allow(dead_code)]
impl Chunk {
  pub fn new(file_id: FileID, length: usize, index: usize) -> Self {
    Self {
      file_id,
      length,
      index,
      ..Default::default()
    }
    .with_new_id()
  }

  pub fn new_with_id(id: ChunkID, file_id: FileID, length: usize, index: usize) -> Self {
    Self {
      file_id,
      length,
      index,
      ..Default::default()
    }
    .with_id(id)
  }

  pub fn with_new_id(mut self) -> Self {
    self.id = *uuid::Uuid::new_v4().as_bytes();
    self
  }

  pub fn with_id(mut self, id: ChunkID) -> Self {
    self.id = id;
    self
  }

  // pub fn with_file_id(mut self, file_id: FileID) -> Self {
  //   self.file_id = file_id;
  //   self
  // }

  // pub fn with_len(mut self, length: usize) -> Self {
  //   self.length = length;
  //   self
  // }

  // pub fn with_index(mut self, index: usize) -> Self {
  //   self.index = index;
  //   self
  // }

  pub fn id(self) -> ChunkID {
    self.id
  }

  pub fn file_id(self) -> FileID {
    self.file_id
  }

  pub fn len(self) -> usize {
    self.length
  }

  pub fn index(self) -> usize {
    self.index
  }

  pub fn encode_as_header(self) -> Header {
    unsafe {
      use alloc::{alloc, dealloc, Layout};
      use mem::size_of;

      let layout = Layout::new::<Header>();
      let ptr = alloc(layout);
      let mut x: isize = 0;

      *(ptr as *mut VersionID) = VERSION;
      x += size_of::<VersionID>() as isize;

      *(ptr.offset(x) as *mut [u8; 4]) = (self.index as u32).to_be_bytes();
      x += size_of::<[u8; 4]>() as isize;

      *(ptr.offset(x) as *mut [u8; 4]) = (self.length as u32).to_be_bytes();
      x += size_of::<[u8; 4]>() as isize;

      *(ptr.offset(x) as *mut ChunkID) = self.id;
      x += size_of::<ChunkID>() as isize;

      *(ptr.offset(x) as *mut FileID) = self.file_id;
      // x += size_of::<FileID>() as isize;

      let buf: Header = *(ptr as *mut Header);

      dealloc(ptr, layout);

      buf
    }
  }

  pub fn decode_from_header(self, buf: &Header) -> Self {
    use mem::size_of;

    let mut x: usize = 0;
    let mut y: usize = size_of::<VersionID>();

    let mut version: VersionID = Default::default();
    version.copy_from_slice(&buf[x..y]);

    x = y;
    y += size_of::<[u8; 4]>();

    let mut index_bs: [u8; 4] = [0; size_of::<[u8; 4]>()];
    index_bs.copy_from_slice(&buf[x..y]);

    x = y;
    y += size_of::<[u8; 4]>();

    let mut length_bs: [u8; 4] = [0; size_of::<[u8; 4]>()];
    length_bs.copy_from_slice(&buf[x..y]);
    // length = u32::from_be_bytes(&buf[x..y]);

    x = y;
    y += size_of::<ChunkID>();

    let mut id: ChunkID = Default::default();
    id.copy_from_slice(&buf[x..y]);

    x = y;
    y += size_of::<FileID>();

    let mut file_id: FileID = Default::default();
    file_id.copy_from_slice(&buf[x..y]);

    Self {
      id,
      file_id,
      index: u32::from_be_bytes(index_bs) as usize,
      length: u32::from_be_bytes(length_bs) as usize,
    }
  }
}
