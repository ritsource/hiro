use std::io;
use std::net;

use crate::interface;
use crate::interface::msg;

use std::io::prelude::*;

pub async fn start_server(addr: net::SocketAddr) -> Result<(), io::Error> {
  let listener = net::TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    println!("- new incoming stream");
    match stream {
      Ok(stream) => handle_stream(stream),
      Err(err) => {
        println!("an error occurred, {}", err);
      }
    }
  }
  Ok(())
}

pub fn handle_stream(mut stream: net::TcpStream) {
  use interface::constants::PROTOCOL_IDENTIFIER_V1;

  let mut buf = [0u8; PROTOCOL_IDENTIFIER_V1.len()];

  match stream.read(&mut buf) {
    Ok(nr) => {
      println!("- read {} bytes", nr);

      let mut pid = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
      pid.copy_from_slice(&buf[0..PROTOCOL_IDENTIFIER_V1.len()]);

      if pid == PROTOCOL_IDENTIFIER_V1 {
        msg::handle_stream(stream);
      } else {
        println!("an error occurred, invalid protocol identifier");
      }
    }
    Err(err) => {
      println!("an error occurred, {}", err);
      println!("terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(net::Shutdown::Both).unwrap();
    }
  }
}
