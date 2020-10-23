use std::fs;
use std::io;
use std::path;

use std::io::Write;

use crate::file::piece::v1 as piece;
#[allow(unused_imports)]
use crate::interface::data;
use crate::interface::payload;

use crate::interface::payload::Payload;

const PIECE_DUMP_DIR: &str = "./data/pieces";

#[allow(dead_code)]
pub fn write_piece_to_fs(p: piece::Piece) -> Result<(), io::Error> {
  let file_path = path::Path::new(PIECE_DUMP_DIR).join(p.id().to_hex_string());

  let mut f = match fs::File::create(&file_path) {
    Ok(f) => f,
    Err(err) => match err.kind() {
      io::ErrorKind::AlreadyExists => match fs::File::open(&file_path) {
        Ok(f) => f,
        Err(err) => return Err(err),
      },
      _ => return Err(err),
    },
  };

  let nw = f.write(&p.data().map_or(vec![], |v| v))?;

  println!(
    "written {} bytes to file: {}",
    nw,
    file_path.to_str().unwrap()
  );

  Ok(())
}

#[allow(dead_code)]
pub fn handle_piece_upload_message(
  pld: payload::PieceUploadReq,
) -> Result<payload::PieceUploadRes, io::Error> {
  write_piece_to_fs(Into::<piece::Piece>::into(pld.data()))?;
  Ok(payload::PieceUploadRes::new(true))
}
