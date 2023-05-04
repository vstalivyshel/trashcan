use crate::kak_cmd::SELF;
use serde::{Deserialize, Serialize};
use std::io::{self, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::process::Stdio;

type Line = Vec<Atom>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Face {
    pub attributes: Vec<String>,
    pub bg: String,
    pub fg: String,
    pub underline: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Atom {
    pub contents: String,
    pub face: Face,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coord {
    pub column: u64,
    pub line: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InfoShow {
    pub title: Line,
    pub content: Vec<Line>,
    pub anchor: Coord,
    pub face: Face,
    pub style: String,
}

impl InfoShow {
    pub fn title_content(&self) -> String {
        let mut cont = String::new();
        for atom in self.title.iter() {
            cont.push_str(&atom.contents)
        }

        cont
    }

    pub fn content(&self) -> String {
        let mut cont = String::new();
        for atom in self.content.iter().flatten() {
            cont.push_str(&atom.contents)
        }

        cont
    }
}

#[derive(Deserialize, Serialize)]
pub struct JsonRpc {
    pub jsonrpc: String,
    pub method: String,
    pub params: InfoShow,
}

pub fn send_to_kak_socket(session: &str, msg: &str) -> Result<(), io::Error> {
    let rntm = std::env::var("XDG_RUNTIME_DIR").expect("runtimedir");
    let socket = std::path::PathBuf::from(rntm).join("kakoune").join(session);
    let mut stream = UnixStream::connect(socket)?;
    stream.write(&encode(msg))?;
    stream.flush()?;

    Ok(())
}

pub fn encode(msg: &str) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(msg.len() + 9);
    result.splice(..0, (msg.len() as u32).to_ne_bytes());
    msg.bytes().for_each(|b| result.push(b));
    result.splice(..0, (result.len() as u32 + 5).to_ne_bytes());
    result.insert(0, b'\x02');

    result
}
