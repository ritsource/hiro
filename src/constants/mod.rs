use std::net;

use crate::interface::types;

pub const PROTOCOL_IDENTIFIER_V1: types::ProtocolIdentifier = *b"HIRO_PROTOCOL_ID_LOL";
pub const PROTOCOL_VERSION_V1: u8 = 1;

// NOTE: this constants are temporary
lazy_static! {
  pub static ref MASTER_IP_ADDR: net::SocketAddr =
    net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 8080);
}
