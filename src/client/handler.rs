use std::io;
use std::net;

use io::{Read, Write};

use crate::constants;
use crate::interface::message;
use crate::interface::payload;
use crate::interface::payload::Payload;

pub fn handle_stream(mut stream: net::TcpStream) -> Result<net::TcpStream, (io::Error, net::TcpStream)> {
  use constants::PROTOCOL_IDENTIFIER_V1;

  let mut buf = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
  if let Err(err) = stream.read(&mut buf) {
    return Err((err, stream));
  }
  let mut proto_id = [0u8; PROTOCOL_IDENTIFIER_V1.len()];
  proto_id.copy_from_slice(&buf);

  if proto_id != constants::PROTOCOL_IDENTIFIER_V1 {
    if let Err(err) = stream.write(&message::gen_buf_for_rpc(
      message::MsgType::Error,
      b"invalid protocol identifier".to_vec(),
    )) {
      return Err((err, stream));
    }
    return Ok(stream);
  }

  match message::Message::from_reader(stream) {
    Ok((msg, stream)) => {
      match msg.msg_type() {
        message::MsgType::Ping => {
          println!("message recieved: a ping");
          Ok(stream)
        }
        message::MsgType::FileRes => match payload::FileRes::from_reader(stream, msg.payload_len() as usize) {
          Ok((payload, mut stream)) => {
            use crate::interface::data;
            use crate::peer::Peer;
            use crate::piece::Piece;
            use std::fs;
            use std::io;
            use std::io::prelude::{Read, Seek, Write};

            let payload: Vec<(data::Piece, data::Peer)> = payload.data();

            // it's going to be something like array of tasks,
            // which we are gonna task.execute() later
            let _arr: Vec<bool> = payload
              .into_iter()
              .map(|(piece, peer)| {
                println!("\nPiece({}) -> Peer({})", piece.id, peer.id);
                println!("Piece: start({}), length({})", piece.start, piece.length);

                let mut f = fs::File::open("data/ipsum.text").expect("couldn't open");

                f.seek(io::SeekFrom::Start(piece.start as u64)).expect("coudn't seek");

                let buf = {
                  let mut total: usize = 0;
                  let mut buf: Vec<u8> = vec![];
                  let mut b = [0u8; 64];

                  while match f.read(&mut b) {
                    Ok(nr) => {
                      total += nr;
                      buf.append(&mut b[..nr].to_vec());

                      if total >= piece.length as usize {
                        false
                      } else if nr == 0 {
                        // return Err((io::Error::new(io::ErrorKind::Other, "unable to read any data from file"), stream));
                        println!("** Error: unable to read any data from file");
                        false
                      } else {
                        true
                      }
                    }
                    Err(err) => {
                      println!("** Error: {}", err);
                      false
                    }
                  } {}

                  buf
                };

                println!("** data read ** \"{}\"", String::from_utf8_lossy(&buf[..]));
                let piece = piece.with_data(Some(buf));

                let piece_id = piece.id;
                let pld = payload::PieceUploadReq::new(piece);
                let piece_req_data = message::gen_buf_for_rpc(message::MsgType::PieceUploadReq, pld.as_vec().unwrap());

                match net::TcpStream::connect(peer.addr) {
                  Ok(mut stream) => match stream.write(&piece_req_data) {
                    Ok(nw) => {
                      println!("successfully written {} bytes", nw);
                      println!("waiting for response ...");
                      if let Err((err, stream)) = handle_stream(stream) {
                        println!("an error occurred, {}", err);
                        println!("terminating connection with {}", stream.peer_addr().unwrap());
                        return false;
                      }
                      println!("Piece({}) has been uploaded successfully", piece_id);
                      true
                    }
                    Err(err) => {
                      println!("** Error: {}", err);
                      false
                    }
                  },
                  Err(err) => {
                    println!("** Error: {}", err);
                    false
                  }
                }
              })
              .collect();

            Ok(stream)
          }
          Err((err, stream)) => Err((err, stream)),
        },
        message::MsgType::PieceUploadRes => {
          match payload::PieceUploadRes::from_reader(stream, msg.payload_len() as usize) {
            Ok((payload, stream)) => {
              println!("file successfully uploaded, response recieved {}", payload.data());
              Ok(stream)
            }
            Err((err, stream)) => Err((err, stream)),
          }
        }
        message::MsgType::Error => {
          println!("message recieved: error");
          Err((io::Error::new(io::ErrorKind::Other, "error message recieved"), stream))
        }
        _ => {
          println!("message recieved: unknown");
          // NOTE: maybe respond with a error message
          Ok(stream)
        }
      }
    } // NOTE: treat io::Error::EOF separately
    Err((err, stream)) => Err((err, stream)),
  }
}
