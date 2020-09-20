use std::io;
use std::net;

use crate::interface::msg;

pub async fn start_server(addr: net::SocketAddr) -> Result<(), io::Error> {
  let listener = net::TcpListener::bind(addr)?;

  for stream in listener.incoming() {
    println!("- new incoming stream");
    match stream {
      Ok(stream) => msg::handle_stream(stream),
      Err(err) => {
        println!("an error occurred, {}", err);
      }
    }
  }
  Ok(())
}
