use std::collections;
use std::io;
use std::net;
use std::path;

use std::io::Write;

use super::controllers;
use super::handler;
use crate::constants;
use crate::file;
use crate::interface::data;
use crate::interface::message;
use crate::interface::payload;
use crate::peer;
use crate::piece;

use crate::interface::payload::Payload;

pub async fn upload_file(path: &path::Path) -> Result<(), io::Error> {
  // 1. send file metadata to master [*]
  // 2. get Piece-Peer map from master [*]
  // 3. read and upload file chunks to workers

  // println!("uploading file, {}", path.file_name());

  let f = controllers::read_file_matadata(path)?;

  match net::TcpStream::connect(*constants::MASTER_IP_ADDR) {
    Ok(mut stream) => match stream.write(&message::gen_buf_for_rpc(
      message::MsgType::FileReq,
      payload::FileReq::new(data::File::from(f)).as_vec().unwrap(),
    )) {
      Ok(nw) => {
        println!("successfully written {} bytes", nw);
        println!("waiting for response ...");

        match handler::handle_stream::<payload::FileRes>(stream).await {
          Ok((Some(payload), stream)) => {
            // master responded with pieces and peer mappings
            use collections::HashMap;
            use peer::Peer;
            use piece::Piece;

            let peer_by_pieces_map: HashMap<Peer, Vec<Piece>> = HashMap::new();
            for (piece, peer) in payload.data().iter() {
              println!("\npieces, {:?}", piece);
              println!("\npeer, {:?}", peer);
            }

            // let peer = Into::<Peer>::into(*peer);
          }
          Ok((None, stream)) => {
            println!("an error occurred, master responsed with invalid data");
            println!("terminating connection with {}", stream.peer_addr().unwrap());
            return Err(io::Error::new(
              io::ErrorKind::Other,
              "master responsed with invalid data",
            ));
          }
          Err((err, stream)) => {
            println!("an error occurred, {}", err);
            println!("terminating connection with {}", stream.peer_addr().unwrap());
            return Err(err);
          }
        }

        Ok(())
      }
      Err(err) => Err(err),
    },
    Err(err) => Err(err),
  }
}
