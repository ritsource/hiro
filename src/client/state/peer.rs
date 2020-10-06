use std::io;

use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::peer;

// #[derive(Clone)]
// pub struct PeerInfo<'p> {
//   peer: &'p peer::Peer,
// }
//
// impl <'p>PeerInfo<'p> {
//   fn new(pr: &'p peer::Peer) -> Self {
//     Self { peer: pr }
//   }
//
//   fn with_peer(&mut self, pr: peer::Peer) -> Self {
//     self.peer = &pr;
//     *self
//   }
//
//   fn peer(self) -> &'p peer::Peer {
//     self.peer
//   }
// }

lazy_static! {
  static ref PEERS: Mutex<HashMap<peer::PeerID, peer::Peer>> = Mutex::new(HashMap::new());
}

// // do I really need all these ???
// #[allow(dead_code)]
// pub async fn get(id: &peer::PeerID) -> Option<&peer::Peer> {
//   let peers = PEERS.lock().await;
//
//   if let Some(pr) = peers.get(id) {
//     Some(pr)
//   } else {
//     None
//   }
// }

#[allow(dead_code)]
pub async fn add(id: &peer::PeerID, pr: peer::Peer) -> Result<(), io::Error> {
  let mut peers = PEERS.lock().await;

  if let None = peers.get(id) {
    // let _ = peers.insert(*id, PeerInfo::new(pr));
    let _ = peers.insert(*id, pr);
    Ok(())
  } else {
    Err(io::Error::new(io::ErrorKind::Other, "Peer with id already exists"))
  }
}

#[allow(dead_code)]
pub async fn update(id: &peer::PeerID, pr: peer::Peer) -> Result<(), io::Error> {
  let mut peers = PEERS.lock().await;

  match peers.get(id) {
    Some(_) => {
      let _ = peers.insert(*id, pr);
      Ok(())
    }
    None => Err(io::Error::new(io::ErrorKind::Other, "Peer with id does not exist")),
  }
}

#[allow(dead_code)]
pub async fn remove(id: &peer::PeerID) {
  let mut peer = PEERS.lock().await;
  let _ = peer.remove(id);
}
