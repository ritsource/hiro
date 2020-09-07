#[allow(unused_imports)]
use crate::file::{self, piece::v1 as piece};
#[allow(unused_imports)]
use crate::interface::data;
use crate::peer;

// lazy_static! {
//   static ref Workers: Vec<data::Peer> = vec![
//     data::Peer::from(peer::Peer::new_worker("192.168.0.248:8080").unwrap()),
//     data::Peer::from(peer::Peer::new_worker("192.168.0.246:8080").unwrap()),
//   ];
// }

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
pub fn calculate_pieces(f: data::File) -> Vec<(data::Piece, data::Peer)> {
  (Into::<file::File>::into(f))
    .pieces()
    .into_iter()
    .zip(get_workers().into_iter())
    .map(|(x, y)| {
      println!("{:?}", x);
      println!("{:?}", y);
      (data::Piece::from(x), data::Peer::from(y))
    })
    .collect()
}
