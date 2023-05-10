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
    fn send(&self, root_path: &str) -> Result<(), bincode::Error> {
        let root_path = Path::new(root_path);
        let socket = if root_path.ends_with(ROOT) {
            root_path.join(SOCKET)
        } else {
            root_path.join(ROOT).join(SOCKET)
        };
        let stream = UnixStream::connect(&socket)?;
        bincode::serialize_into(stream, self)?;

        Ok(())
    }

    fn send_err<E: ToString>(&self, msg: &str, error: &E) -> Result<(), io::Error>{
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
    pid_file: PathBuf,
    socket: UnixListener,
}

impl GluaServer {
    fn setup(root_path: Option<&String>) -> Result<Self, io::Error> {
        let root = if let Some(specified_root) = root_path {
            tempfile::Builder::new()
                .rand_bytes(0)
                .prefix(ROOT)
                .tempdir_in(Path::new(specified_root))
        } else {
            tempfile::Builder::new()
                .rand_bytes(0)
                .prefix(ROOT)
                .tempdir()
        }?;

        let root_path = root.path();
        let pid_file = root_path.join(&SELF.and(".pid"));
        let socket_path = root_path.join(SOCKET);
        let socket = UnixListener::bind(&socket_path)?;

        Ok(GluaServer {
            root,
            pid_file,
            socket,
        })
    }

    fn run(self) -> Result<(), io::Error> {
        let mut last_request = Request::Continue;
        for stream in self.socket.incoming() {
            if let Err(ref stream_err) = stream {
                let _ = last_request.send_err("Failed to read request from stream", &stream_err);
            }

            let request = match bincode::deserialize_from::<_, Request>(stream.unwrap()) {
                Ok(r) => {
                    last_request = r.clone();
                    r
                },
                Err(des_err) => {
                    let _ = last_request.send_err("Failed to deserialize client request", &des_err);
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
                Stop =>  break,
            }
        }

        Ok(())
    }
}

fn main() {
    let self_cmd = std::env::current_exe().unwrap();
    let self_cmd = self_cmd.to_str().unwrap().to_owned();
    let mut args = std::env::args().skip(1).collect::<VecDeque<String>>();
    if args.len() < 1 {
        println!("fail {SELF}::Error: Wrong argument count");
        process::exit(69);
    }
    let sub = args.get(0).unwrap();

    match args.len() {
        2 if sub.starts_with("kill") => Request::Stop.send(&args.get(1).unwrap()).unwrap(),
        1 | 2 if sub.starts_with("init") => {
            let root = args.get(1);
            match GluaServer::setup(root) {
                Err(io_err) => println!("fail {SELF}::Error: Failed to spawn server: {io_err}"),
                Ok(server) => {
                    let pid_file = server.pid_file.clone();
                    let root = server.root.path();
                    let socket_path = root.to_str().unwrap().to_owned();

                    println!(
                        "{init_cmd}",
                        init_cmd = kak_init_cmd(&self_cmd, &socket_path)
                    );
                    println!("echo {socket_path}");

                    use daemonize::Stdio;
                    if let Err(d_err) = daemonize::Daemonize::new()
                        .pid_file(pid_file)
                        .working_directory(&std::env::current_dir().unwrap())
                        .start()
                    {
                         println!("fail {SELF}::Error: Failed to daemonize: {d_err}");
                        process::exit(69);
                    }

                    server.run().unwrap();
                }
            }
        }
        4 => {
            let server_root = args.pop_front().unwrap();

            if let Err(io_err) = Request::ExecLua(ClientData {
                session: args.pop_front().unwrap(),
                client: args.pop_front().unwrap(),
                chunck: args.pop_back().unwrap().into(),
                chunck_args: args.into_iter().collect::<Vec<String>>(),
            })
            .send(&server_root)
            {
                println!("fail {SELF}::Error: Failed to send lua chunck: {io_err}");
            }
        }
        _ => println!("fail {SELF}::Error: Wrong argument count"),
    }
}
