use crate::file::{self, piece::v1 as piece};
use crate::interface::data;
use crate::interface::message::payload::{self, Payload};
use crate::peer;

#[allow(dead_code)]
fn get_workers() -> Vec<peer::Peer> {
  vec![
    peer::Peer::new_worker("192.168.0.246:8080").unwrap(),
    peer::Peer::new_worker("192.168.0.246:8080").unwrap(),
  ]
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
  payload::FileRes::new(
    (Into::<file::File>::into(payload.data()))
      .pieces()
      .into_iter()
      .zip(get_workers().into_iter())
      .map(|(x, y)| {
        println!("{:?}", x);
        println!("{:?}", y);
        (data::Piece::from(x), data::Peer::from(y))
      })
      .collect(),
  )
}
