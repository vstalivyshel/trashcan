#[macro_use]
mod lua;
mod kak_cmd;
mod utils;
use crate::kak_cmd::{Cmd, EVACL, EXEC, SELF};
use crate::lua::{lua_prelude, Lua};
use crate::utils::*;
use log::{debug, info};
use std::io::BufRead;

struct GluaClient {
    session: String,
    lua_state: Lua,
    stream: std::io::BufReader<std::process::ChildStdout>,
    output_buffer: Vec<u8>,
}

impl GluaClient {
    fn connect(session: &str) -> Self {
        let lua_state = lua_prelude(session)
            .map_err(|lua_err| {
                fuck(&f!("Lua State failed to initialize:" lua_err.to_string().dqt()))
            })
            .unwrap();

        let stream = connect_to(session)
            .map_err(|io_err| {
                fuck(&f!("Failed to connect to the kakoune session:" io_err.to_string().dqt()))
            })
            .unwrap();

        let output_buffer = Vec::<u8>::new();

        Self {
            session: session.to_string(),
            lua_state,
            stream,
            output_buffer,
        }
    }

    fn send_to_socket(&self, msg: &str) {
        let _ = send_to_kak_socket(&self.session, msg);
    }

    fn read_request(&mut self) -> Result<JsonRpc, serde_json::Error> {
        self.stream
            .read_until(b'\n', &mut self.output_buffer)
            .expect("read json output");
        let inc_req = serde_json::from_slice::<JsonRpc>(&self.output_buffer);
        self.output_buffer.clear();

        inc_req
    }
}

fn run() {
    let session = std::env::args().skip(1).next();
    if session.is_none() {
        fuck("No session name supplied");
    }

    let mut server = GluaClient::connect(&session.unwrap());

    loop {
        let received = server.read_request();

        if let Err(parse_err) = received {
            if parse_err.is_eof() {
                break;
            }
            continue;
        }

        let info = received.unwrap().params;
        server.send_to_socket(&f!(EVACL SELF).and_kakqt(f!(EXEC "<esc>")));

        let chunck = info.content();
        if chunck.is_empty() {
            continue;
        }

        let client = info.title_content();

        info!("InfoShow.title.content: \"{client}\"");
        info!("InfoShow.content.content: \"{chunck}\"");
        debug!("InfoShow: \n{info:?}");

        // info!("Request from client: \"{client}\"");
        // info!("With arguments: {args:?}");
    }
}

fn main() {
    env_logger::init();
    run();
}
