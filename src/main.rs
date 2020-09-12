extern crate hiro;

use std::env;
use std::net;

use std::io::prelude::*;

#[allow(unused_imports)]
use hiro::file;
#[allow(unused_imports)]
use hiro::file::piece::v1 as piece;
#[allow(unused_imports)]
use hiro::interface::data;
#[allow(unused_imports)]
use hiro::interface::msg;
use hiro::master;

fn main() {
  // master::controllers::calculate_pieces(msg::GetPiecesFromFileRequest::new(data::File::from(file::File::new(
  //   file::piece::DEFAULT_PIECE_SIZE,
  //   None,
  // ))));
  //

  let master_addr: net::SocketAddr = "127.0.0.1:8080".parse().unwrap();

  let args: Vec<String> = env::args().collect();

  if args.len() > 1 && args[1] == "--master" {
    println!("Starting master at {}", &master_addr);
    master::start_server(master_addr).unwrap();
  } else if args.len() > 1 && args[1] == "--client" {
    println!("Starting client");

    match net::TcpStream::connect(master_addr) {
      Ok(mut stream) => match stream.write(b"Hello world!") {
        Ok(nw) => {
          println!("successfully written {} bytes", nw);
        }
        Err(err) => {
          println!("Error: couldn't write to connection: {}", err);
        }
      },
      Err(err) => {
        println!("Error: failed to connect: {}", err);
      }
    }
  }
}
