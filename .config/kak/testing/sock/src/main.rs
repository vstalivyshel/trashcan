mod kakoune;
mod lua;
mod utils;
use kakoune::*;
use utils::*;
use lua::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    env, io,
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
};
use tempfile::TempDir;

pub const SELF: &str = "GLUA";
pub const SOCKET: &str = "GLUA.socket";
pub const ROOT: &str = "GLUA.root";
pub const SOCK_HANDLER: &str = "GLUA_socket_root";
pub const LIST_FILE: &str = "GLUA.list";

// TODO: Don't echo path after born
enum Do {
    Kill(Option<String>),
    Spawn(Option<String>),
    Send(Request, String),
    SendSync(Request, String),
    WrongArgs,
}

#[derive(Serialize, Deserialize, Clone)]
enum Request {
    ExecLua(ClientData),
    Continue,
    Stop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    session: String,
    client: String,
    chunck: String,
    chunck_args: Vec<String>,
}

struct GluaServer {
    lua: Lua,
    root: TempDir,
    root_path: PathBuf,
    pid_file: PathBuf,
    socket: UnixListener,
}

impl Do {
    fn this(mut args: VecDeque<String>) -> Self {
        let arg_len = args.len();

        if arg_len < 1 {
            return WrongArgs;
        }

        let sub = args.pop_front().unwrap();
        let sub = sub.as_str();
        use Do::*;
        match sub {
            "kill" => Kill(args.pop_front()),
            "spawn" => Spawn(args.pop_front()),
            "send" | "sendsync" if arg_len > 4 => {
                let server_root = args.pop_front().unwrap();

                let req = Request::ExecLua(ClientData {
                    session: args.pop_front().unwrap(),
                    client: args.pop_front().unwrap(),
                    chunck: args.pop_back().unwrap(),
                    chunck_args: args.into_iter().collect::<Vec<String>>(),
                });

                if sub.contains("sync") {
                    SendSync(req, server_root)
                } else {
                    Send(req, server_root)
                }
            }
            _ => WrongArgs,
        }
    }
}

impl Request {
    fn send_to<P: AsRef<Path>>(&self, root_path: &P) -> Result<(), bincode::Error> {
        let root_path = root_path.as_ref();
        let path = Path::new(root_path);
        let root_path = root_path.to_str().unwrap();
        let socket = if root_path.ends_with(SOCKET) {
            path.to_path_buf()
        } else if root_path.ends_with(ROOT) {
            path.join(SOCKET)
        } else {
            path.with_extension(ROOT).join(SOCKET)
        };

        bincode::serialize_into(UnixStream::connect(&socket)?, self)
    }

    fn try_send_err<E: ToString>(&self, msg: &str, error: &E) -> Result<(), io::Error> {
        if let Request::ExecLua(ref d) = self {
            let session = &d.session;
            let client = &d.client;
            kak_throw_error(session, client, msg, &error.to_string())?;
        }

        Ok(())
    }
}

fn find_and_kill(specified_path: Option<&String>) -> Result<Vec<String>, bincode::Error> {
    let path = match specified_path {
        Some(p) if !p.is_empty() => Path::new(&p).canonicalize()?,
        _ => env::temp_dir(),
    };
    let target = path.to_str().unwrap().to_string();

    let mut removed = Vec::new();
    if path.is_dir() && !target.ends_with(ROOT) {
        for found in find_file_in(path, |p| p.to_str().unwrap().contains(ROOT))? {
            Request::Stop.send_to(&found)?;
            removed.push(found.to_str().unwrap().to_string());
        }
    } else {
        Request::Stop.send_to(&path)?;
        removed.push(target);
    }

    Ok(removed)
}

impl GluaServer {
    fn setup(path: Option<String>) -> Result<Self, mlua::Error> {
        let root_suffix = &format!(".{ROOT}");
        let root = match path {
            Some(specified_path) if !specified_path.is_empty() => {
                let path = Path::new(&specified_path);
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
            }
            _ => tempfile::Builder::new().suffix(root_suffix).tempdir(),
        }?;

        let root_path = root.path().canonicalize()?;
        let pid_file = root_path.join(&SELF.and(".pid"));
        let socket_path = root_path.join(SOCKET);
        let socket = UnixListener::bind(&socket_path)?;
        let lua = Lua::new();
        lua.prelude(root_path.to_str().unwrap().to_string())?;

        Ok(GluaServer {
            lua,
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
                continue;
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
                    if let Err(lua_err) = self.lua.load_data(ses_data) {
                        let _ = last_request.try_send_err("Lua error", &lua_err);
                    }
                }
                Continue => continue,
                Stop => break,
            }
        }

        Ok(())
    }
}

fn main() {
    use Do::*;
    match Do::this(std::env::args().skip(1).collect::<VecDeque<String>>()) {
        Spawn(target) => match GluaServer::setup(target) {
            Err(io_err) => println!("fail {SELF}::Error: Failed to spawn server: {io_err}"),
            Ok(server) => {
                let socket_root = server.root_path.to_str().unwrap();
                println!("{socket_root}");

                if let Err(d_err) = daemonize::Daemonize::new()
                    .pid_file(&server.pid_file)
                    .working_directory(&std::env::current_dir().unwrap())
                    .start()
                {
                    println!("fail {SELF}::Error: Failed to daemonize: {d_err}");
                } else {
                    let _ = server.run();
                }
            }
        },
        Kill(target) => match find_and_kill(target.as_ref()) {
            Ok(killed_targets) => {
                let path = if let Some(path) = target {
                    if let Ok(p) = Path::new(&path).canonicalize() {
                        p.to_str().unwrap().to_string()
                    } else {
                        path
                    }
                } else {
                    "your temp directory".to_string()
                };

                if killed_targets.is_empty() {
                    print_info(f!("There is nothing to kill in" path));
                } else if killed_targets.len() == 1 {
                    print_info(f!("Server root in" path "has been killed"));
                } else {
                    let dead = killed_targets
                        .into_iter()
                        .map(|x| x.dqt().and(", "))
                        .collect::<String>();
                    print_info(f!("Killed a couple:" dead))
                }
            }
            Err(io_err) => {
                if let Some(p) = target {
                    println!("fail {SELF}::Error: Failed to kill {p}: {io_err}");
                } else {
                    println!("fail {SELF}::Error: Failed to kill server: {io_err}");
                }
            }
        },
        Send(req, target) => {
            if let Err(io_err) = req.send_to(&target) {
                println!("fail {SELF}::Error: Failed to send lua chunck: {io_err}");
            }
        }
        SendSync(req, target) => {
            if let Err(io_err) = req.send_to(&target) {
                println!("fail {SELF}::Error: Failed to send lua chunck: {io_err}");
            }
        }
        WrongArgs => println!("fail {SELF}::Error: Wrong option"),
    }
}
