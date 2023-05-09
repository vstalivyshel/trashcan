mod kakoune;
mod utils;
use kakoune::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process;
use tempfile::TempDir;
use utils::*;

pub const SELF: &str = "GLUA";
pub const MAIN_CMD: &str = "glua-eval-test";

#[derive(Serialize, Deserialize)]
enum Request {
    ExecLua(ClientData),
    Stop,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientData {
    session: String,
    client: String,
    chunck: String,
    chunck_args: Vec<String>,
}

struct GluaServer {
    root: TempDir,
    pid_file: PathBuf,
    socket_path: PathBuf,
    socket: UnixListener,
}

impl GluaServer {
    fn setup(root_path: Option<&String>) -> Result<Self, io::Error> {
        let root_name = &SELF.and(".root");

        let root = if let Some(specified_root) = root_path {
            tempfile::Builder::new()
                .prefix(root_name)
                .rand_bytes(0)
                .tempdir_in(Path::new(specified_root))
        } else {
            tempfile::Builder::new().prefix(root_name).tempdir()
        }?;

        let root_path = root.path();
        let pid_file = root_path.join(&SELF.and(".pid"));
        let socket_path = root_path.join(&SELF.and(".socket"));
        let socket = UnixListener::bind(&root_path.join(&socket_path))?;

        Ok(GluaServer {
            root,
            pid_file,
            socket_path,
            socket,
        })
    }

    fn run(self) -> Result<(), io::Error> {
        let mut incoming = self.socket.incoming();
        for stream in incoming {
            let mut stream = stream.unwrap();
            let request: Request = bincode::deserialize_from(stream).unwrap();
            use Request::*;
            match request {
                ExecLua(ses_data) => {
                    todo!("exec lua")
                }
                Stop => {
                    break;
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let self_cmd = std::env::current_exe().unwrap();
    let self_cmd = self_cmd.to_str().unwrap();
    let mut args = std::env::args().skip(1).collect::<VecDeque<String>>();
    if args.len() < 1 {
        println!("fail {SELF}::Error: Wrong argument count");
        process::exit(69);
    }
    let sub = args.get(0).unwrap();

    match args.len() {
        1 | 2 if sub.starts_with("init") => {
            let root = args.get(1);
            let server = GluaServer::setup(root).unwrap();
            println!(
                "{init_cmd}",
                init_cmd = kak_init_cmd(&self_cmd, &server.socket_path.to_str().unwrap())
            );
            server.run().unwrap();
            // let cwd = std::env::current_dir().unwrap();
            // let daemon = daemonize::Daemonize::new()
            //     .pid_file(&server.pid_file)
            //     .working_directory(&cwd)
            //     .start()
            //     .unwrap();
        }
        2 if sub.starts_with("kill") => {
            let server = args.get(1).unwrap();
            let stream = UnixStream::connect(&server).unwrap();
            bincode::serialize_into(stream, &Request::Stop).unwrap();
        }
        4 => {
            let socket = args.pop_front().unwrap();

            let data = ClientData {
                session: args.pop_front().unwrap(),
                client: args.pop_front().unwrap(),
                chunck: args.pop_back().unwrap().into(),
                chunck_args: args.into_iter().collect::<Vec<String>>(),
            };

            let mut stream = UnixStream::connect(&socket).unwrap();
            bincode::serialize_into(stream, &Request::ExecLua(data)).unwrap();
        }
        _ => println!("fail {SELF}::Error: Your {SELF} command have wrong argument count!"),
    }
}
