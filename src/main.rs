extern crate hiro;

use std::env;
use std::path;

use hiro::client;
use hiro::env as hiro_env;
use hiro::master;
use hiro::worker;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  let listener_addr = hiro_env::get_listener_socket_addr();

  if args.len() > 1 && args[1] == "--master" {
    println!("master listening on address: {}", &listener_addr);
    master::start_server(listener_addr).await.unwrap();
  } else if args.len() > 1 && args[1] == "--worker" {
    worker::start_server(listener_addr).await.unwrap();
  } else if args.len() > 1 && args[1] == "--client" {
    let file_path = if args.len() > 2 {
      path::Path::new(&args[2])
    } else {
      panic!("no file provided")
    };

    println!("Starting client");
    match client::upload_file(file_path).await {
      Ok(_) => {
        println!("file has successfully been uploaded",);
      }
      Err(err) => {
        println!("Error: {}", err);
      }
    }
  }
}
