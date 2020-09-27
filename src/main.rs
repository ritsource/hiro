extern crate hiro;

use std::env;
use std::path;

use hiro::client;
use hiro::constants;
use hiro::master;
use hiro::worker;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 && args[1] == "--master" {
    println!("Starting master at {}", &*constants::MASTER_IP_ADDR);
    master::start_server(*constants::MASTER_IP_ADDR).await.unwrap();
  } else if args.len() > 1 && args[1] == "--worker" {
    println!("Hello I'm the worker");
    worker::start_server(*constants::WORKER_SERVER_ADDR).await.unwrap();
  } else if args.len() > 1 && args[1] == "--client" {
    let file_path = if args.len() > 2 {
      path::Path::new(&args[2])
    } else {
      panic!("no file provided")
    };

    println!("Starting client");
    match client::upload_file(file_path) {
      Ok(_) => {
        println!("file has successfully been uploaded",);
      }
      Err(err) => {
        println!("Error: {}", err);
      }
    }
  }
}
