use super::data;
#[allow(unused_imports)]
use crate::file::piece::v1 as piece;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum MsgPayload {
  FileMetaData(data::File),

  // one possibility is to let the client send worker data about what other workers to clone to
  // this will result in less messeging over all, but master won't have a control over it. so no.
  // PiecesAndWorkers(Vec<(data::Piece, Vec<data::Peer>)>),
  PieceAndPeer(Vec<(data::Piece, data::Peer)>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MsgType {
  X,
  Y,
  Z,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
  pub sender: data::Peer,
  pub receiver: data::Peer,
  pub message_type: MsgType,
  pub payload: Option<MsgPayload>,
}
