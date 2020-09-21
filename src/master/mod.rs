pub mod controllers;

use std::io;
use std::net;

use crate::interface;

pub async fn start_server(addr: net::SocketAddr) -> Result<(), io::Error> {
  let listener = net::TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    println!("- new incoming stream");
    match stream {
      Ok(stream) => {
        if let Err((err, stream)) = interface::handle_stream(stream) {
          println!("an error occurred, {}", err);
          println!("terminating connection with {}", stream.peer_addr().unwrap());
          stream.shutdown(net::Shutdown::Both).unwrap();
        }
      }
      Err(err) => {
        println!("an error occurred, {}", err);
      }
    }
  }
  Ok(())
}
