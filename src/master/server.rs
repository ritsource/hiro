use std::io;
use std::net;

use std::io::prelude::*;

pub async fn start_server(addr: net::SocketAddr) -> Result<(), io::Error> {
  let listener = net::TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => handle_stream(stream),
      Err(err) => {
        println!("Error: {}", err);
      }
    }
  }
  Ok(())
}

pub fn handle_stream(mut stream: net::TcpStream) {
  let mut rcounter = 0;
  let mut buf = [0u8; 8];
  let mut msglen = 0u32;
  let mut start = true;

  while match stream.read(&mut buf) {
    Ok(nr) => {
      println!("- read {} bytes", nr);
      rcounter += nr;

      if start && nr >= 4 {
        msglen = u32::from_be_bytes((|| {
          let mut x = [0u8; 4];
          x.copy_from_slice(&buf[0..4]);
          x
        })());

        println!("Message length: {}", msglen);

        true
      } else if !start {
        println!("Request: {}", String::from_utf8_lossy(&buf[..nr]));

        if nr == 0 {
          false
        } else {
          true
        }
      } else {
        false
      }
    }
    Err(err) => {
      println!("an error occurred, {}", err);
      println!("terminating connection with {}", stream.peer_addr().unwrap());
      stream.shutdown(net::Shutdown::Both).unwrap();

      false
    }
  } {}
}
