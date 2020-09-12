use super::data;
#[allow(unused_imports)]
use crate::file::piece::v1 as piece;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::HashMap;

pub trait MessagePayload<'de>: Serialize + Deserialize<'de> {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPiecesFromFileRequest(data::File);

impl GetPiecesFromFileRequest {
  pub fn new(data: data::File) -> Self {
    Self(data)
  }

  pub fn data(self) -> data::File {
    self.0
  }
}

impl MessagePayload<'_> for GetPiecesFromFileRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPiecesFromFileResponse(Vec<(data::Piece, data::Peer)>);

impl GetPiecesFromFileResponse {
  pub fn new(data: Vec<(data::Piece, data::Peer)>) -> Self {
    Self(data)
  }

  pub fn data(self) -> Vec<(data::Piece, data::Peer)> {
    self.0
  }
}

impl MessagePayload<'_> for GetPiecesFromFileResponse {}

pub struct Message {
  pub sender: data::Peer,
  pub receiver: data::Peer,
}
