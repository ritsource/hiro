extern crate hiro;

use std::env;
use std::net;
use std::path;

use hiro::client;
use hiro::constants;
use hiro::master;
use hiro::worker;

fn parse_socket_addr(args: Vec<String>) -> Option<net::SocketAddr> {
  if args.len() > 3 && (args[2] == "-p" || args[2] == "--port") && args.len() >= 4 {
    let port = match args[3].parse::<u16>() {
      Ok(p) => p,
      Err(_) => return None,
    };
    Some(net::SocketAddr::new(
      net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
      port,
    ))
  } else {
    None
  }
}

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 && args[1] == "--master" {
    let master_socket_addr = parse_socket_addr(args).unwrap_or(*constants::MASTER_IP_ADDR);
    println!("master listening on address: {}", &master_socket_addr);

    master::start_server(master_socket_addr).await.unwrap();
  } else if args.len() > 1 && args[1] == "--worker" {
    let worker_socket_addr = parse_socket_addr(args).unwrap_or(*constants::WORKER_SERVER_ADDR);
    println!("worker listening on address: {}", &worker_socket_addr);

    worker::start_server(worker_socket_addr).await.unwrap();
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
