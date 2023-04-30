use std::io::{self, Write};
use std::os::unix::net::UnixStream;

pub fn send_to_kak_socket(session: &str, msg: &str) -> Result<(), io::Error> {
    let mut stream = conncet_to_kak_socket(&session)?;
    stream.write(&encode(msg))?;
    stream.flush()?;

    Ok(())
}

pub fn conncet_to_kak_socket(session: &str) -> Result<UnixStream, io::Error> {
    // TODO: Need to handle a result
    let rntimedir = std::env::var("XDG_RUNTIME_DIR").expect("runtime path");
    let socket_path = std::path::Path::new(&rntimedir)
        .join("kakoune")
        .join(session);

    UnixStream::connect(socket_path)
}

pub fn encode(msg: &str) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(msg.len() + 9);
    result.splice(..0, (msg.len() as u32).to_ne_bytes());
    msg.bytes().for_each(|b| result.push(b));
    result.splice(..0, (result.len() as u32 + 5).to_ne_bytes());
    result.insert(0, b'\x02');

    result
}
