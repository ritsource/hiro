use crate::file::piece::v1 as piece;

// NOTE: For copy of data - for every chunk there will
// be a primary worker and multiple secondary worker.

// 1. Chunks are calculated and initialized by master
// 2. Then it's sent to the client where it will write
// the data to the corresponding workers
// 3. Worker will handle the data given by client

#[allow(dead_code)]
pub struct Piece {
  header: piece::Header,
  data: Vec<u8>,
}

#[allow(dead_code)]
impl Piece {}
