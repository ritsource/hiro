use crate::env;
use crate::file::{self, piece::v1 as piece};
use crate::interface::data;
use crate::interface::payload;
use crate::interface::payload::Payload;
use crate::peer;

#[allow(dead_code)]
fn get_workers() -> Vec<peer::Peer> {
  env::get_worker_socket_addrs()
    .iter()
    .map(|addr| peer::Peer::new(*addr, peer::PeerType::Worker))
    .collect()
}

#[allow(dead_code)]
fn assign_pieces_to_peers(ps: Vec<piece::Piece>) -> Vec<(piece::Piece, Vec<peer::Peer>)> {
  ps.into_iter().map(|p| (p, get_workers())).collect()
}

#[allow(dead_code)]
pub fn calculate_pieces(payload: payload::FileReq) -> payload::FileRes {
  // NOTE: make sure workers.len() can never be 0
  let workers: Vec<data::Peer> = get_workers()
    .into_iter()
    .map(|w| data::Peer::from(w))
    .collect();
  let mut n = 0;
  const MAX_COUNT_OF_WORKER_ADDRS_FOR_A_PIECE: usize = 3;

  let res = (Into::<file::File>::into(payload.data()))
    .pieces()
    .into_iter()
    .map(|piece| {
      // NOTE: make it more understandable, try
      // if it's possible with iter
      n = get_next_index(n, workers.len());
      let n1 = get_next_index(n + 1, workers.len());
      let n2 = get_next_index(n + 2, workers.len());
      (
        data::Piece::from(piece),
        vec![workers[n], workers[n1], workers[n2]],
      )
    })
    .collect();

  payload::FileRes::new(res)
}

fn get_next_index(idx: usize, max: usize) -> usize {
  if idx < 0 || idx + 1 > max {
    0
  } else {
    idx + 1
  }
}
