use std::collections::HashMap;
use tokio::sync::Mutex;

use crate::peer;

lazy_static! {
  static ref PEERS: Mutex<HashMap<peer::PeerID, peer::Peer>> = Mutex::new(HashMap::new());
}
