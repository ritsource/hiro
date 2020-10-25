mod helpers;

use std::env;
use std::net;

// NOTE: cache the Vec<Opiton<net::SocketAddr>>
// NOTE: find a way to return Vec<net::SocketAddr>
pub fn get_worker_socket_addrs() -> Vec<net::SocketAddr> {
  let args = env::args().collect();

  let worker_addrs = helpers::parse_worker_socket_addrs_from_args(args)
    .unwrap_or(helpers::parse_worker_socket_addrs_from_env().unwrap_or(vec![]))
    .iter()
    .filter_map(|opt| *opt)
    .collect();

  println!("worker socket addresses: {:?}", worker_addrs);
  worker_addrs
}

pub fn get_listener_socket_addr() -> net::SocketAddr {
  let args = env::args().collect();

  let listener_addr = helpers::parse_listener_socket_addr_from_args(args).unwrap_or(
    helpers::parse_listener_socket_addr_from_env().unwrap_or(net::SocketAddr::new(
      net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
      8080,
    )),
  );

  println!("listener socket address: {:?}", listener_addr);
  listener_addr
}

pub fn get_master_socket_addr() -> net::SocketAddr {
  let args = env::args().collect();

  let master_addr = helpers::parse_master_socket_addr_from_args(args).unwrap_or(
    helpers::parse_master_socket_addr_from_env().unwrap_or(net::SocketAddr::new(
      net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
      8080,
    )),
  );

  println!("master socket address: {:?}", master_addr);
  master_addr
}
