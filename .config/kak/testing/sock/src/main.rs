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
pub const SOCKET: &str = "GLUA.socket";
pub const ROOT: &str = "GLUA.root";

#[derive(Serialize, Deserialize, Clone)]
enum Request {
    ExecLua(ClientData),
    Continue,
    Stop,
}

impl Request {
    fn send_to(&self, root_path: &str) -> Result<(), bincode::Error> {
        let path = Path::new(root_path);
        let socket = if root_path.contains(ROOT) {
            path.join(SOCKET)
        } else {
            path.with_extension(ROOT).join(SOCKET)
        };

        bincode::serialize_into(UnixStream::connect(&socket)?, self)
    }

    fn try_send_err<E: ToString>(&self, msg: &str, error: &E) -> Result<(), io::Error> {
        if let Request::ExecLua(ref d) = self {
            let session = &d.session;
            let client = &d.session;
            kak_throw_error(session, client, msg, error.to_string())?;
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClientData {
    session: String,
    client: String,
    chunck: String,
    chunck_args: Vec<String>,
}

struct GluaServer {
    root: TempDir,
    root_path: PathBuf,
    pid_file: PathBuf,
    socket: UnixListener,
}

impl GluaServer {
    fn setup(path: Option<&String>) -> Result<Self, io::Error> {
        let root_suffix = &".".and(ROOT);
        let root = if let Some(specified_path) = path {
            let path = Path::new(specified_path);
            if path.is_dir() {
                tempfile::Builder::new()
                    .rand_bytes(0)
                    .prefix(root_suffix)
                    .tempdir_in(&path)
            } else {
                let dir_name = path.file_name().unwrap();
                let path = path.parent().unwrap();
                tempfile::Builder::new()
                    .rand_bytes(0)
                    .prefix(dir_name)
                    .suffix(root_suffix)
                    .tempdir_in(&path)
            }
        } else {
            tempfile::Builder::new().suffix(root_suffix).tempdir()
        }?;

        let root_path = root.path().canonicalize()?;
        let pid_file = root_path.join(&SELF.and(".pid"));
        let socket_path = root_path.join(SOCKET);
        let socket = UnixListener::bind(&socket_path)?;

        Ok(GluaServer {
            root,
            root_path,
            pid_file,
            socket,
        })
    }

    fn run(self) -> Result<(), io::Error> {
        let mut last_request = Request::Continue;
        for stream in self.socket.incoming() {
            if let Err(ref stream_err) = stream {
                let _ =
                    last_request.try_send_err("Failed to read request from stream", &stream_err);
            }

            let request = match bincode::deserialize_from::<_, Request>(stream.unwrap()) {
                Ok(r) => {
                    last_request = r.clone();
                    r
                }
                Err(des_err) => {
                    let _ =
                        last_request.try_send_err("Failed to deserialize client request", &des_err);
                    Request::Continue
                }
            };

            use Request::*;
            match request {
                ExecLua(ses_data) => {
                    let session = &ses_data.session;
                    let client = &ses_data.client;
                    let cmd = &ses_data.chunck;
                    let _ = kak_send_client(session, client, cmd);
                }
                Continue => continue,
                Stop => break,
            }
        }

        Ok(())
    }
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<VecDeque<String>>();
    if args.len() < 1 {
        println!("fail {SELF}::Error: Wrong argument count");
        process::exit(69);
    }

    let self_cmd = std::env::current_exe().unwrap();
    let self_cmd = self_cmd.to_str().unwrap().to_owned();
    let sub = args.get(0).unwrap();

    match args.len() {
        1 | 2 if sub.starts_with("kill") => {
            if let Some(ref specified_server) = args.get(1) {
                if let Err(io_err) = search_and_kill(specified_server) {
                    println!("fail {SELF}:Error: Failed to kill specified server: {io_err}");
                }
            } else {
                let temp = std::env::temp_dir();
                let temp = temp.to_str().unwrap();
                if let Err(io_err) = search_and_kill(temp) {
                    println!(
                        "fail {SELF}:Error: Failed to kill unnamed server in {temp} : {io_err}"
                    );
                }
            }
        }

        1 | 2 if sub.starts_with("init") => match GluaServer::setup(args.get(1)) {
            Err(io_err) => println!("fail {SELF}::Error: Failed to spawn server: {io_err}"),
            Ok(server) => {
                let socket_root = server.root_path.to_str().unwrap();

                println!(
                    "{init_cmd}",
                    init_cmd = kak_init_cmd(&self_cmd, socket_root),
                );
                print_info(f!("Born in" socket_root));

                if let Err(d_err) = daemonize::Daemonize::new()
                    .pid_file(&server.pid_file)
                    .working_directory(&std::env::current_dir().unwrap())
                    .start()
                {
                    println!("fail {SELF}::Error: Failed to daemonize: {d_err}");
                    process::exit(69);
                }

                server.run().unwrap();
            }
        },
        4 => {
            let server_root = args.pop_front().unwrap();

            if let Err(io_err) = Request::ExecLua(ClientData {
                session: args.pop_front().unwrap(),
                client: args.pop_front().unwrap(),
                chunck: args.pop_back().unwrap().into(),
                chunck_args: args.into_iter().collect::<Vec<String>>(),
            })
            .send_to(&server_root)
            {
                println!("fail {SELF}::Error: Failed to send lua chunck: {io_err}");
            }
        }
        _ => println!("fail {SELF}::Error: Wrong argument count"),
    }
}

fn search_and_kill(specified_path: &str) -> Result<(), bincode::Error> {
    let path = Path::new(specified_path);
    let mut found_any = false;
    if path.is_dir() {
        for entry in path.read_dir()? {
            let entry_path = entry?.path();
            let path = entry_path.to_str().unwrap();
            if path.contains(ROOT) {
                Request::Stop.send_to(&path)?;
                print_info(f!("Server root in" path "has been killed"));
                found_any = true;
            }
        }
        if !found_any {
            print_info(f!("There is nothing to kill in" specified_path ));
        }

        Ok(())
    } else {
        Request::Stop.send_to(specified_path)
    }
}

fn print_info<S: std::fmt::Display>(msg: S) {
    println!("echo -debug {SELF}::Info: {msg}");
    println!("echo -markup {{Information}}{SELF}::Info: {msg}");
}
