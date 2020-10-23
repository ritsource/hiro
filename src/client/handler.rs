use std::any;
use std::io;
use std::net;

use io::{Read, Write};

use crate::constants;
use crate::interface::message;
use crate::interface::payload;
use crate::interface::payload::Payload;

pub async fn handle_stream<D>(
  mut stream: net::TcpStream,
) -> Result<(Option<D>, net::TcpStream), (io::Error, net::TcpStream)>
where
  D: any::Any + Clone + std::fmt::Debug,
{
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
    return Ok((None, stream));
  }

  match message::MessageMetadata::from_reader(stream) {
    Ok((msg, stream)) => {
      match msg.msg_type() {
        message::MsgType::Ping => {
          println!("message recieved: a ping");
          Ok((None, stream))
        }
        message::MsgType::FileRes => {
          match payload::FileRes::from_reader(stream, msg.payload_len() as usize) {
            Ok((pld, stream)) => {
              let pld_any = &pld as &dyn any::Any;
              match pld_any.downcast_ref::<D>() {
                Some(pld) => Ok((Some(pld.clone()), stream)),
                None => Ok((None, stream)),
              }
            }
            Err((err, stream)) => Err((err, stream)),
          }
        }
        message::MsgType::PieceUploadRes => {
          match payload::PieceUploadRes::from_reader(stream, msg.payload_len() as usize) {
            Ok((payload, stream)) => {
              println!(
                "file successfully uploaded, response recieved {}",
                payload.data()
              );
              Ok((None, stream))
            }
            Err((err, stream)) => Err((err, stream)),
          }
        }
        message::MsgType::Error => {
          println!("message recieved: error");
          Err((
            io::Error::new(io::ErrorKind::Other, "error message recieved"),
            stream,
          ))
        }
        _ => {
          println!("message recieved: unknown");
          // NOTE: maybe respond with a error message
          Ok((None, stream))
        }
      }
    } // NOTE: treat io::Error::EOF separately
    Err((err, stream)) => Err((err, stream)),
  }
}
