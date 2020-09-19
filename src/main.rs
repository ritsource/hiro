extern crate hiro;

use std::env;
use std::net;

use std::io::prelude::*;

#[allow(unused_imports)]
use hiro::file;
#[allow(unused_imports)]
use hiro::file::piece::v1 as piece;
#[allow(unused_imports)]
use hiro::interface;
#[allow(unused_imports)]
use hiro::interface::data;
#[allow(unused_imports)]
use hiro::interface::msg;
use hiro::master;

#[tokio::main]
async fn main() {
  // master::controllers::calculate_pieces(msg::GetPiecesFromFileRequest::new(data::File::from(file::File::new(
  //   file::piece::DEFAULT_PIECE_SIZE,
  //   None,
  // ))));
  //

  let master_addr: net::SocketAddr = "127.0.0.1:8080".parse().unwrap();

  let args: Vec<String> = env::args().collect();

  if args.len() > 1 && args[1] == "--master" {
    println!("Starting master at {}", &master_addr);
    master::start_server(master_addr).await.unwrap();
  } else if args.len() > 1 && args[1] == "--client" {
    println!("Starting client");

    match net::TcpStream::connect(master_addr) {
      Ok(mut stream) => match stream.write(&file_message((|| {
        data::File::from(file::File::new(file::piece::DEFAULT_PIECE_SIZE, None))
      })())) {
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

#[allow(dead_code)]
fn ping_message() -> Vec<u8> {
  let mut buf = interface::constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(
    msg::message_type_id_from_message_type(msg::MessageType::default())
      .to_be_bytes()
      .iter(),
  );
  buf
}

#[allow(dead_code)]
fn file_message(file: interface::data::File) -> Vec<u8> {
  let mut buf = interface::constants::PROTOCOL_IDENTIFIER_V1.to_vec();
  buf.extend(
    msg::message_type_id_from_message_type(msg::MessageType::PiecesAndPeersForFileRequest)
      .to_be_bytes()
      .iter(),
  );
  buf.extend(msg::types::PiecesAndPeersForFileRequest::new(file).as_vec().unwrap());
  buf
}
