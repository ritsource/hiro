use std::alloc;
use std::fmt;
use std::mem;
use uuid;

use crate::file::file::FileID;
use crate::file::piece::VersionID;
use crate::id::v1 as id;

pub const VERSION: VersionID = [0u8; 3];
pub const HEADER_LEN: usize =
  mem::size_of::<VersionID>() + mem::size_of::<[u8; 4]>() + mem::size_of::<[u8; 4]>() + mem::size_of::<PieceID>();

pub type Header = [u8; HEADER_LEN];

pub type PieceID = id::ID;

#[derive(Default, Debug)]
pub struct Piece {
  pub id: PieceID,
  pub file_id: FileID,
  pub index: usize,
  pub length: usize,
  data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Piece {
  pub fn new(file_id: FileID, length: usize, index: usize) -> Self {
    Self {
      file_id,
      length,
      index,
      ..Default::default()
    }
    .with_new_id()
  }

  pub fn new_with_id(id: PieceID, file_id: FileID, length: usize, index: usize) -> Self {
    Self {
      file_id,
      length,
      index,
      ..Default::default()
    }
    .with_id(id)
  }

  pub fn with_new_id(mut self) -> Self {
    self.id = PieceID::new();
    self
  }

  pub fn with_id(mut self, id: PieceID) -> Self {
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

  pub fn with_data(mut self, data: Option<Vec<u8>>) -> Self {
    self.data = data;
    self
  }

  pub fn id(&self) -> PieceID {
    self.id
  }

  pub fn file_id(&self) -> FileID {
    self.file_id
  }

  pub fn len(&self) -> usize {
    self.length
  }

  pub fn index(&self) -> usize {
    self.index
  }

  pub fn data(self) -> Option<Vec<u8>> {
    self.data
  }

  pub fn has_data(&self) -> bool {
    self.data != None
  }

  fn encode_as_header(self) -> Header {
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

      *(ptr.offset(x) as *mut PieceID) = self.id;
      x += size_of::<PieceID>() as isize;

      *(ptr.offset(x) as *mut FileID) = self.file_id;
      // x += size_of::<FileID>() as isize;

      let buf: Header = *(ptr as *mut Header);

      dealloc(ptr, layout);

      buf
    }
  }

  fn decode_from_header(buf: &Header) -> Self {
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
    y += size_of::<PieceID>();

    let mut id_buf: uuid::Bytes = Default::default();
    id_buf.copy_from_slice(&buf[x..y]);
    let id = PieceID::from_bytes(id_buf);

    x = y;
    y += size_of::<FileID>();

    let mut file_id_buf: uuid::Bytes = Default::default();
    file_id_buf.copy_from_slice(&buf[x..y]);
    let file_id = FileID::from_bytes(file_id_buf);

    Self {
      id,
      file_id,
      index: u32::from_be_bytes(index_bs) as usize,
      length: u32::from_be_bytes(length_bs) as usize,
      ..Default::default()
    }
  }

  pub fn serialize(self) -> Vec<u8> {
    self.encode_as_header().to_vec()
  }

  pub fn deserialize(buf: Vec<u8>) -> Result<Self, DeserializationInputLengthError> {
    use std::convert::TryInto;

    Ok(Self::decode_from_header(
      &({
        let boxed_arr: Box<Header> = match buf.into_boxed_slice().try_into() {
          Ok(ba) => ba,
          Err(_) => {
            return Err(DeserializationInputLengthError::new());
          }
        };
        *boxed_arr
      }),
    ))
  }
}

#[derive(Debug, Clone)]
pub struct DeserializationInputLengthError(());

impl DeserializationInputLengthError {
  fn new() -> Self {
    Self(())
  }
  fn description(&self) -> &str {
    "deserialization input is not long enough"
  }
}

impl fmt::Display for DeserializationInputLengthError {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.write_str(self.description())
  }
}

impl std::error::Error for DeserializationInputLengthError {
  fn description(&self) -> &str {
    self.description()
  }
}
