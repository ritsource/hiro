extern crate hiro;

use hiro::file;
use hiro::file::piece::v1 as piece;
use hiro::interface::data;
use hiro::master;

fn main() {
  master::controllers::calculate_pieces(data::File::from(file::File::new(file::piece::DEFAULT_PIECE_SIZE, None)));

  println!("Hello, world!");
}
