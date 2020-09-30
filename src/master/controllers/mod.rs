use crate::file::{self, piece::v1 as piece};
use crate::interface::data;
use crate::interface::payload;
use crate::interface::payload::Payload;
use crate::peer;

#[allow(dead_code)]
fn get_workers() -> Vec<peer::Peer> {
  // vec![
  //   peer::Peer::new_worker("192.168.0.246:8080").unwrap(),
  //   peer::Peer::new_worker("192.168.0.246:8080").unwrap(),
  // ]
  vec![peer::Peer::new_worker("127.0.0.1:8090").unwrap()]
}

// fn get_workers() -> Vec<data::Peer> {
//   vec![
//     peer::Peer::new_worker("192.168.0.246:8080").unwrap(),
//     peer::Peer::new_worker("192.168.0.246:8080").unwrap(),
//   ]
//   .into_iter()
//   .map(|pp| data::Peer::from(pp))
//   .collect()
// }

#[allow(dead_code)]
fn assign_pieces_to_peers(ps: Vec<piece::Piece>) -> Vec<(piece::Piece, Vec<peer::Peer>)> {
  ps.into_iter().map(|p| (p, get_workers())).collect()
}

#[allow(dead_code)]
pub fn calculate_pieces(payload: payload::FileReq) -> payload::FileRes {
  // NOTE: make sure workers.len() can never be 0
  let workers: Vec<data::Peer> = get_workers().into_iter().map(|w| data::Peer::from(w)).collect();
  let mut y = 0;

  payload::FileRes::new(
    (Into::<file::File>::into(payload.data()))
      .pieces()
      .into_iter()
      .map(|piece| {
        if y > workers.len() - 1 {
          y = 0
        }
        (data::Piece::from(piece), data::Peer::from(workers[y]))
      })
      .collect(),
  )
}
