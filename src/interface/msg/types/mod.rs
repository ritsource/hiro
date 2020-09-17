use super::data;

pub struct PiecesAndPeersForFileRequest(data::File);

impl PiecesAndPeersForFileRequest {
  pub fn new(data: data::File) -> Self {
    Self(data)
  }

  pub fn data(self) -> data::File {
    self.0
  }
}

pub struct PiecesAndPeersForFileResponse(Vec<(data::Piece, data::Peer)>);

impl PiecesAndPeersForFileResponse {
  pub fn new(data: Vec<(data::Piece, data::Peer)>) -> Self {
    Self(data)
  }

  pub fn data(self) -> Vec<(data::Piece, data::Peer)> {
    self.0
  }
}
