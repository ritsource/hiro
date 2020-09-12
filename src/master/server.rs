use std::io;
use std::net::{SocketAddr, TcpListener};

use std::io::prelude::*;

pub fn start_server(addr: SocketAddr) -> Result<(), io::Error> {
  let listener = TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
          Ok(nr) => {
            println!("read {} bytes", nr);
          }
          Err(err) => {
            println!("Error: {}", err);
          }
        }

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
      }
      Err(err) => {
        println!("Error: {}", err);
      }
    }
  }
  Ok(())
}
