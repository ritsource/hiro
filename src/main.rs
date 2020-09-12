extern crate hiro;

use hiro::file;
#[allow(unused_imports)]
use hiro::file::piece::v1 as piece;
use hiro::interface::data;
use hiro::interface::msg;
use hiro::master;

fn main() {
  master::controllers::calculate_pieces(msg::GetPiecesFromFileRequest::new(data::File::from(file::File::new(
    file::piece::DEFAULT_PIECE_SIZE,
    None,
  ))));

  println!("Hello, world!");
}
