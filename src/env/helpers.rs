use std::env;
use std::io;
use std::net;

// static mut LISTENER_PORT: Opiton<net::SocketAddr = "127.0.0.1:8080".parse().unwrap();
// static mut WORKER_SOCKET_ADDRS: Vec<net::SocketAddr> = "127.0.0.1:".parse().unwrap();
// static mut MASTER_SOCKET_ADDR: net::SocketAdr = "127.0.0.1:8080".parse().unwrap();

enum Flag {
  Port,
  Workers,
  Master,
}

impl Flag {
  fn ids(&self) -> (String, String) {
    match self {
      Self::Port => ("-p".to_owned(), "--port".to_owned()),
      Self::Workers => ("-w".to_owned(), "--workers".to_owned()),
      Self::Master => ("-m".to_owned(), "--master".to_owned()),
    }
  }
}

fn get_arg_values_by_flag(args: Vec<String>, flag: Flag) -> Option<String> {
  for (i, arg) in args.iter().enumerate() {
    if *arg == flag.ids().0 || *arg == flag.ids().1 {
      if arg.len() > i {
        return Some(args[i + 1].clone());
      } else {
        return None;
      }
    }
  }

  None
}

pub fn parse_listener_socket_addr_from_args(args: Vec<String>) -> Option<net::SocketAddr> {
  let port = get_arg_values_by_flag(args, Flag::Port)?;

  match port.parse::<u16>() {
    Ok(port) => Some(net::SocketAddr::new(
      net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
      port,
    )),
    Err(_) => None,
  }
}

pub fn parse_listener_socket_addr_from_env() -> Option<net::SocketAddr> {
  match env::var("PORT") {
    Ok(port_str) => match port_str.parse::<u16>() {
      Ok(port) => Some(net::SocketAddr::new(
        net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
        port,
      )),
      Err(_) => None,
    },
    Err(_) => None,
  }
}

pub fn parse_worker_socket_addrs_from_env() -> Result<Vec<Option<net::SocketAddr>>, env::VarError> {
  match env::var("WORKERS") {
    Ok(addrs) => Ok(
      addrs
        .split(",")
        .map(|addr| match addr.parse::<net::SocketAddr>() {
          Ok(sa) => Some(sa),
          Err(_) => None,
        })
        .collect(),
    ),
    Err(err) => Err(err),
  }
}

pub fn parse_worker_socket_addrs_from_args(
  args: Vec<String>,
) -> Result<Vec<Option<net::SocketAddr>>, io::Error> {
  let worker_addrs = match get_arg_values_by_flag(args, Flag::Workers) {
    Some(was) => was,
    None => return Err(io::Error::new(io::ErrorKind::Other, "arguement not passed")),
  };

  Ok(
    worker_addrs
      .split(",")
      .map(|arg| match arg.parse::<net::SocketAddr>() {
        Ok(sa) => Some(sa),
        Err(_) => None,
      })
      .collect(),
  )
}

pub fn parse_master_socket_addr_from_args(args: Vec<String>) -> Option<net::SocketAddr> {
  // cargo run -- --client -f "file.txt" -m 127.0.0.1:8080
  // cargo run -- --client "file.txt" -m 127.0.0.1:8080
  // cargo run -- --master -p 8080 -w 127.0.0.1:5050,127.0.0.1:5051
  // cargo run -- --master -w 127.0.0.1:5050,127.0.0.1:5051 -p 8080

  let master_addr = get_arg_values_by_flag(args, Flag::Workers)?;

  match master_addr.parse::<net::SocketAddr>() {
    Ok(addr) => Some(addr),
    Err(_) => None,
  }
}

pub fn parse_master_socket_addr_from_env() -> Option<net::SocketAddr> {
  match env::var("MASTER") {
    Ok(master_addr) => match master_addr.parse::<net::SocketAddr>() {
      Ok(addr) => Some(addr),
      Err(_) => None,
    },
    Err(_) => None,
  }
}
