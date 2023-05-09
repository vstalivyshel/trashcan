mod kak_cmd;
// mod lua;
mod utils;
use crate::kak_cmd::{evacl, throw_error, Cmd};
use crate::lua::{Lua, LuaServer, CLIENT, ROOT, SES};
use crate::utils::*;

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Write, BufRead, BufReader, Read},
    os::unix::net::{UnixStream, UnixListener},
    path::{Path, PathBuf},
    process::{self, Stdio},
    thread::sleep,
    time::Duration,
};

pub const SELF: &str = "GLUA";
pub const VAL_SEP: &str = "§";
pub const SOCK_HANDLER: &str = "glua_socket_path";

// TODO: val fifo need to have random name
// TODO: maybe i don't even need to create any fifo: socket can do everything

struct GluaServer {
    session: String,
    root_dir: tempfile::TempDir,
    buffer: Vec<u8>,
    socket: UnixListener,
    lua: Lua,
}

impl GluaServer {
    pub fn setup(client: String, session: String) -> Result<Self, mlua::Error> {
        let it = &format!("{SELF}_{session}");
        let root_dir = tempfile::Builder::new().prefix(it).tempdir()?;
        let root_path = root_dir.path();
        let socket_path = root_path.join(it);
        let socket = UnixListener::bind(&socket_path)?;

        let lua = Lua::new();
        lua.server_prelude();
        let to_lua_root_dir = root_path.to_str().unwrap().to_string();
        lua.set_data::<String>(ROOT, to_lua_root_dir)?;
        lua.set_data::<String>(SES, session.clone())?;
        lua.set_data::<String>(CLIENT, client)?;
        lua.kak_eval(f!("declare-option str" SOCK_HANDLER socket_path.to_str().unwrap().to_string()))?;

        Ok(GluaServer {
            session,
            root_dir,
            buffer: Vec::<u8>::new(),
            socket,
            lua,
        })
    }

    fn run(&mut self) -> Result<(), io::Error> {
        for stream in self.socket.incoming() {
            let mut received = String::new();
            stream?.read_to_string(&mut received)?;

            if received.starts_with("stop") {
                break;
            } 

            if let Err(lua_err) = self.lua.exec(received) {
                let _ = self.lua.kak_eval(kak_cmd::throw_error(
                    SELF.and("::Error::Chunck Error: See debug!"),
                    lua_err.to_string(),
                ));
            }
        }

        Ok(())
    }
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<VecDeque<String>>();

    if args.len() < 3 {
        eprintln!("fail {SELF}::Error: Wronge argument count!");
        process::exit(69);
    }

    let socket_path = args.pop_front().unwrap();
    let session = args.pop_front().unwrap();
    let client = args.pop_front().unwrap();
    let chunck = args.pop_back();

    if socket_path.starts_with("init") {
        let mut server = match GluaServer::setup(client.clone(), session.clone()) {
            Err(setup_failed) => {
                let _ = send_to_kak_socket(&session, &evacl(
                    &client,
                    &kak_cmd::throw_error(
                        SELF.and("::Error: Server setup failed! See debug!"),
                        setup_failed.to_string(),
                    ),
                ));
                process::exit(69);
            }
            Ok(nice) => nice,
        };

        if let Err(io_err) = server.run() {
            let _ = send_to_kak_socket(&session, &evacl(
                &client,
                &kak_cmd::throw_error(
                    SELF.and("::Error: Some error. See debug"),
                    io_err.to_string(),
                ),
            ));
        }

        process::exit(0);
    }

    if let Some(ch) = chunck {
        let mut stream = match UnixStream::connect(&socket_path) {
            Ok(s) => s,
            Err(sock_err) => {
                eprintln!("fail {SELF}::Error: Failed to connect to socket:\n{sock_err}");
                process::exit(69);
            }
        };

        let mut msg = String::new();

        msg.push_str(&session);
        msg.push_str(VAL_SEP);

        msg.push_str(&client);
        msg.push_str(VAL_SEP);

        msg.push_str(&ch);
        msg.push_str(VAL_SEP);

        for arg in args {
            msg.push_str(&arg);
            msg.push_str(VAL_SEP);
        }

        let _ = msg.pop();
        if let Err(io_err) = stream.write_all(msg.as_bytes()) {
            eprintln!( "fail {SELF}::Error: Failed to write to the socket:\n{io_err}" );
            process::exit(69);
        }
    }

}
