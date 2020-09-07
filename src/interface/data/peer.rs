use crate::peer::{self, PeerID, PeerType};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Peer {
  pub id: PeerID,
  pub peer_type: PeerType,
  pub addr: SocketAddr,
}

impl From<peer::Peer> for Peer {
  fn from(p: peer::Peer) -> Self {
    Self {
      id: p.id,
      peer_type: p.peer_type,
      addr: p.addr,
    }
  }
}

impl Into<peer::Peer> for Peer {
  fn into(self) -> peer::Peer {
    peer::Peer {
      id: self.id,
      peer_type: self.peer_type,
      addr: self.addr,
      stream: Default::default(),
    }
  }
}
