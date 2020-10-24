use std::collections;
use std::io;
use std::net;
use std::path;

use std::io::Write;

use super::controllers;
use super::handler;
use crate::env;
#[allow(unused_imports)]
use crate::file;
use crate::interface::data;
use crate::interface::message;
use crate::interface::payload;
use crate::peer;
use crate::piece;

use crate::interface::payload::Payload;

// struct PieceC {}

pub async fn upload_file(path: &path::Path) -> Result<(), io::Error> {
  // 1. send file metadata to master [*]
  // 2. get Piece-Peer map from master [*]
  // 3. read and upload file chunks to workers

  // println!("uploading file, {}", path.file_name());

  let f = controllers::read_file_matadata(path)?;

  match net::TcpStream::connect(env::get_master_socket_addr()) {
    Ok(mut stream) => match stream.write(&message::gen_buf_for_rpc(
      message::MsgType::FileReq,
      payload::FileReq::new(data::File::from(f)).as_vec().unwrap(),
    )) {
      Ok(nw) => {
        println!("successfully written {} bytes", nw);
        println!("waiting for response ...");

        match handler::handle_stream::<payload::FileRes>(stream).await {
          Ok((Some(payload), _stream)) => {
            // master responded with pieces and peer mappings
            use collections::HashMap;
            use peer::Peer;
            use piece::Piece;

            let _peer_by_pieces_map: HashMap<Peer, Vec<Piece>> = HashMap::new();
            let _piece_state_map: HashMap<Piece, (bool, Piece)> = HashMap::new();

            for (piece, peer) in payload.data().iter() {
              println!("\npieces, {:?}", piece);
              println!("\npeer, {:?}", peer);

              let mut peer = Into::<Peer>::into(*peer);

              // if !peer.is_connected() {
              //   peer.connect()?;
              // }

              // if let Some(mut peer_stream) = peer.connection() {
              //   let msg_buf = message::message_buffer_from_payload(
              //     message::MsgType::PieceUploadReq,
              //     payload::PieceUploadReq::new(piece.clone()),
              //   )?;

              //   match peer_stream.write(&msg_buf) {
              //     Ok(nw) => {
              //       println!("successfully written {} bytes", nw);
              //       println!("waiting for response ...");

              //       match handler::handle_stream::<payload::PieceUploadRes>(peer_stream).await {
              //         Ok((Some(_resp), _peer_stream)) => {}
              //         Ok((None, _peer_stream)) => {}
              //         Err(_err) => {}
              //       }
              //     }
              //     Err(err) => {
              //       println!("an error occurred, {}", err);
              //     }
              //   }

              //   // Err that represents worker failure
              //   // Err(X::ErrorKind::WorkerFailure) => {},
              // }

              match peer
                .write_message(
                  message::MsgType::PieceUploadReq,
                  payload::PieceUploadReq::new(piece.clone()),
                )
                .await
              {
                Ok(nw) => {
                  println!("successfully written {} bytes", nw);
                  println!("waiting for response ...");
                }
                Err(err) => {
                  println!("an error occurred, {}", err);
                }
              }
            }
          }
          Ok((None, stream)) => {
            println!("an error occurred, master responsed with invalid data");
            println!(
              "terminating connection with {}",
              stream.peer_addr().unwrap()
            );
            return Err(io::Error::new(
              io::ErrorKind::Other,
              "master responsed with invalid data",
            ));
          }
          Err((err, stream)) => {
            println!("an error occurred, {}", err);
            println!(
              "terminating connection with {}",
              stream.peer_addr().unwrap()
            );
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

#[allow(dead_code)]
async fn handle_peer() -> Result<(), io::Error> {
  Ok(())
}
