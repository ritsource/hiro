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
      Ok(mut stream) => match stream.write(&msg::build_message_buffer(
        msg::MessageType::PiecesAndPeersForFileRequest,
        msg::types::PiecesAndPeersForFileRequest::new(data::File::from(file::File::new(
          file::piece::DEFAULT_PIECE_SIZE,
          None,
        )))
        .as_vec()
        .unwrap(),
      )) {
        Ok(nw) => {
          println!("successfully written {} bytes", nw);
          println!("... waiting for response ...");

          if let Err((err, stream)) = interface::handle_stream(stream) {
            println!("an error occurred, {}", err);
            println!("terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(net::Shutdown::Both).unwrap();
          }
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
