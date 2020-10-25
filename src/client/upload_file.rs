use std::collections;
use std::io;
use std::net;
use std::path;
use std::sync;

use std::io::Write;

use tokio::sync as tokio_sync;

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
            use peer::{Peer, PeerID};
            use piece::Piece;

            // const MAX_ALLOWED_PEER_FAILURES: i32 = 3;

            // let peer_map: sync::Arc<tokio_sync::Mutex<HashMap<PeerID, (Peer, bool)>>> =
            //   Default::default();
            // let peer_by_pieces_map: sync::Arc<tokio_sync::Mutex<HashMap<Peer, Vec<Piece>>>> =
            //   Default::default();
            // let piece_state_map: sync::Arc<tokio_sync::Mutex<HashMap<Piece, (bool, Piece)>>> =
            //   Default::default();
            // let peer_failure_count_map: sync::Arc<
            //   tokio_sync::Mutex<HashMap<PeerID, (bool, Piece)>>,
            // > = Default::default();

            for (piece, peers) in payload.data().iter() {
              // let mut peers = Into::<Peer>::into(*peer);
              let peers: Vec<Peer> = peers.iter().map(|p| Into::<Peer>::into(*p)).collect();

              // NOTE: this happens asynchronously
              for mut peer in peers {
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

            // NOTE: should not be here, but anyways.
            // if every chunk contains a version number
            // (that might be useful for file synchronoization)
            // it would that be harder (riskier) to recreate
            // a file from it (the solution needs to be in the
            // configurations)
            // for now let's not think about file synchronoization

            // .. peers - x, y, x
            // .. pieces - x (a, b, c), y (e, f, g), z (h, k)
            // for p in pieces {
            //   if peer is not busy, upload ...
            //   else next pirce ...
            // }
            //
            // for each peer {
            //   async for each piece in peer/piece {
            //     upload ->
            //       Ok(uploaded)
            //       Err(failed, and add the failure to a failure_tracking list)
            //       // later (in some other subroutine) we can use the list
            //       // to determie if worker's faulty
            //   }
            // }

            // for (piece, peer) in payload.data().iter() {
            //   let mut peer = Into::<Peer>::into(*peer);

            //   //   if !peer.is_connected() {
            //   //     peer.connect()?;
            //   //   }

            //   //   if let Some(mut peer_stream) = peer.connection() {
            //   //     let msg_buf = message::message_buffer_from_payload(
            //   //       message::MsgType::PieceUploadReq,
            //   //       payload::PieceUploadReq::new(piece.clone()),
            //   //     )?;

            //   //     match peer_stream.write(&msg_buf) {
            //   //       Ok(nw) => {
            //   //         println!("successfully written {} bytes", nw);
            //   //         println!("waiting for response ...");

            //   //         match handler::handle_stream::<payload::PieceUploadRes>(peer_stream).await {
            //   //           Ok((Some(resp), peer_stream)) => {}
            //   //           Ok((None, peer_stream)) => {}
            //   //           // NOTE: probably a worker failure
            //   //           Err(err) => {}
            //   //         }
            //   //       }
            //   //       Err(err) => {
            //   //         println!("an error occurred, {}", err);
            //   //       }
            //   //     }

            //   //     // NOTE: handle Err that represents worker failure
            //   //     // Err(X::ErrorKind::WorkerFailure) => {},
            //   //   }

            //   match peer
            //     .write_message(
            //       message::MsgType::PieceUploadReq,
            //       payload::PieceUploadReq::new(piece.clone()),
            //     )
            //     .await
            //   {
            //     Ok(nw) => {
            //       println!("successfully written {} bytes", nw);
            //       println!("waiting for response ...");
            //     }
            //     Err(err) => {
            //       println!("an error occurred, {}", err);
            //     }
            //   }
            // }
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

// async fn write_piece(
//   piece: data::Piece,
//   mut peer: peer::Peer,
// ) -> Result<(peer::Peer, bool), (peer::Peer, io::Error)> {

// async fn write_piece(piece: data::Piece, peer: &mut peer::Peer) -> Result<usize, io::Error> {
//   match peer
//     .write_message(
//       message::MsgType::PieceUploadReq,
//       payload::PieceUploadReq::new(piece.clone()),
//     )
//     .await
//   {
//     Ok(nw) => {
//       println!("successfully written {} bytes", nw);
//       println!("waiting for response ...");
//     }
//     Err(err) => {
//       println!("an error occurred, {}", err);
//     }
//   }
//
//   // if !peer.is_connected() {
//   //   if let Err(err) = peer.connect() {
//   //     return Err((peer, err));
//   //   }
//   // }
//
//   // if let Some(mut peer_stream) = peer.connection() {
//   //   let msg_buf = match message::message_buffer_from_payload(
//   //     message::MsgType::PieceUploadReq,
//   //     payload::PieceUploadReq::new(piece),
//   //   ) {
//   //     Ok(buf) => buf,
//   //     Err(err) => return Err((peer, err)),
//   //   };
//
//   //   match peer_stream.write(&msg_buf) {
//   //     Ok(nw) => {
//   //       println!("successfully written {} bytes", nw);
//   //       println!("waiting for response ...");
//
//   //       match handler::handle_stream::<payload::PieceUploadRes>(peer_stream).await {
//   //         Ok((Some(resp), peer_stream)) => {}
//   //         Ok((None, peer_stream)) => {}
//   //         // NOTE: probably a worker failure
//   //         Err(err) => {}
//   //       }
//   //     }
//   //     Err(err) => {
//   //       println!("an error occurred, {}", err);
//   //     }
//   //   }
//
//   //   // NOTE: handle Err that represents worker failure
//   //   // Err(X::ErrorKind::WorkerFailure) => {},
//   // }
//   Ok((peer, true))
// }
