pub mod controllers;
pub mod handler;

use std::io;
use std::net;
use std::path;

use std::io::Write;

use crate::constants;
use crate::interface::data;
use crate::interface::message;
use crate::interface::payload;

use crate::interface::payload::Payload;

// upload file
// upload multiple files
// download file
// download multiple files
// edit file (metadata)
// edit multiple files (metadata)
// delete file
// delete multiple files

pub fn upload_file(path: &path::Path) -> Result<(), io::Error> {
  // 1. send file metadata to master
  // 2. get Piece-Peer map from master
  // 3. read and upload file chunks to workers

  // println!("uploading file, {}", path.file_name());

  let file = controllers::read_file_matadata(path)?;

  match net::TcpStream::connect(*constants::MASTER_IP_ADDR) {
    Ok(mut stream) => match stream.write(&message::gen_buf_for_rpc(
      message::MsgType::FileReq,
      payload::FileReq::new(data::File::from(file)).as_vec().unwrap(),
    )) {
      Ok(nw) => {
        println!("successfully written {} bytes", nw);
        println!("... waiting for response ...");

        if let Err((err, stream)) = handler::handle_stream(stream) {
          println!("an error occurred, {}", err);
          println!("terminating connection with {}", stream.peer_addr().unwrap());
          stream.shutdown(net::Shutdown::Both).unwrap();
        }
      }
      Err(err) => {
        println!("Error: couldn't write to connection: {}", err);
      }
    },
    Err(err) => {
      println!("Error: failed to connect: {}", err);
    }
  }

  Ok(())
}
