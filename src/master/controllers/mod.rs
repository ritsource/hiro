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

  const MAX_COUNT_OF_WORKER_ADDRS_FOR_A_PIECE: usize = 3;

  let max_count_of_worker_addrs_for_a_piece =
    if workers.len() < MAX_COUNT_OF_WORKER_ADDRS_FOR_A_PIECE {
      workers.len()
    } else {
      MAX_COUNT_OF_WORKER_ADDRS_FOR_A_PIECE
    };

  let res = (Into::<file::File>::into(payload.data()))
    .pieces()
    .into_iter()
    .map(|piece| {
      // let idx_arr = (0..max_count_of_worker_addrs_for_a_piece- 1).collect::<Vec<usize>>();
      (
        data::Piece::from(piece),
        (0..max_count_of_worker_addrs_for_a_piece - 1)
          .map(|i| workers[i])
          .collect(),
      )
    })
    .collect();

  payload::FileRes::new(res)
}
