mod kakoune;
mod lua;
mod utils;
use kakoune::*;
use lua::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    env, io,
    os::unix::{
        fs::FileTypeExt,
        net::{UnixListener, UnixStream},
    },
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use utils::*;

pub const SELF: &str = "GLUA";
pub const SOCKET: &str = "GLUA.socket";
pub const ROOT: &str = "GLUA.root";
pub const SOCK_HANDLER: &str = "GLUA_socket_root";
pub const LIST_FILE: &str = "GLUA.list";

/// TODO: How to return a generic error?
/// TODO: Fix send_to() using CustomError()

enum Do {
    Kill(Option<String>),
    Spawn(Option<String>),
    Eval(Request, String),
    WrongArgs,
}

#[derive(Serialize, Deserialize, Clone)]
enum Request {
    LuaExec(ClientData),
    Return(Vec<String>),
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
            "eval" if arg_len > 4 => {
                let server_root = args.pop_front().unwrap();
                let this = ClientData {
                    session: args.pop_front().unwrap(),
                    client: args.pop_front().unwrap(),
                    chunck: args.pop_back().unwrap(),
                    chunck_args: args.into_iter().collect::<Vec<String>>(),
                };

                Eval(Request::LuaExec(this), server_root)
            }
            _ => WrongArgs,
        }
    }
}

impl Request {
    fn send_to<P: AsRef<Path>>(&self, root_path: &P) -> Result<(), bincode::Error> {
        let socket = find_socket_in(root_path)?;
        let stream = UnixStream::connect(&socket)?;
        bincode::serialize_into(&stream, self)
    }

    fn send_and_recv<P: AsRef<Path>>(&self, root_path: &P) -> Result<Request, bincode::Error> {
        let socket = find_socket_in(root_path)?;
        let stream = UnixStream::connect(&socket)?;
        bincode::serialize_into(&stream, self)?;
        bincode::deserialize_from::<_, Request>(&stream)
    }

    fn try_send_err<E: ToString>(&self, msg: &str, error: &E) -> Result<(), io::Error> {
        if let Request::LuaExec(ref d) = self {
            let session = &d.session;
            let client = &d.client;
            kak_throw_error(session, client, msg, &error.to_string())?;
        }

        Ok(())
    }
}

fn find_socket_in<P: AsRef<Path>>(path: &P) -> Result<Found, io::Error> {
    let root_path = path.as_ref();

    let path = Path::new(root_path);
    let root_path = root_path.to_str().unwrap();
    let socket = if root_path.ends_with(SOCKET) {
        Found::One(path.to_path_buf())
    } else if root_path.ends_with(ROOT) {
        Found::One(path.join(SOCKET))
    } else {
        find_file_in(path, |f| f.with_extension(ROOT).join(SOCKET).is_socket())?
    };

    Ok(socket)
}

fn find_and_kill(specified_path: Option<&String>) -> Result<String, bincode::Error> {
    let path = match specified_path {
        Some(p) if !p.is_empty() => Path::new(&p).to_path_buf(),
        _ => env::temp_dir(),
    };
    use Found::*;
    let found = find_socket_in(&path)?;
    match &found {
        One(sock) => {
            Request::Stop.send_to(sock)?;
        }
        Couple(socks) => {
            for s in socks {
                Request::Stop.send_to(s)?;
            }
        }
        None => {}
    };

    Ok(found)
}

struct GluaServer {
    lua: Lua,
    root: TempDir,
    root_path: PathBuf,
    pid_file: PathBuf,
    socket: UnixListener,
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

    fn run(self) {
        let mut last_request = Request::Continue;
        for stream in self.socket.incoming() {
            if let Err(ref stream_err) = stream {
                let _ =
                    last_request.try_send_err("Failed to read request from stream", &stream_err);
                continue;
            }
            let stream = stream.unwrap();

            let request = match bincode::deserialize_from::<_, Request>(&stream) {
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
                LuaExec(this) => match self.lua.call_chunk(this) {
                    Ok(ret_vals) => {
                        let req_back = if ret_vals.is_empty() {
                            Continue
                        } else {
                            Return(ret_vals)
                        };

                        if let Err(de_err) = bincode::serialize_into(&stream, &req_back) {
                            let _ = last_request
                                .try_send_err("Failed to receive return values from lua", &de_err);
                        }
                    }
                    Err(lua_err) => {
                        let _ = last_request.try_send_err("Lua error", &lua_err);
                        continue;
                    }
                },
                Continue => continue,
                Stop => break,
                Return(_) => unreachable!("what?"),
            }
        }
    }
}

fn main() {
    use Do::*;
    match Do::this(std::env::args().skip(1).collect::<VecDeque<String>>()) {
        Spawn(target) => match GluaServer::setup(target) {
            Err(io_err) => println!("fail {SELF}::Error: Failed to spawn server: {io_err}"),
            Ok(server) => {
                let socket_root = server.root_path.to_str().unwrap();
                print_info(f!("Born in" socket_root.dqt()));

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
            Ok(killed) => match killed {
                None if target.is_some() => println!("There is nothing to kill in {}", target.unwrap()),
                None => println!("There is nothing to kill in temp directory"),
                _ => println!("Killed: {}", killed),
            },
            Err(io_err) => if target.is_some() {
                println!("fail {SELF}::Error: Failed to kill {p}: {io_err}", p = target.unwrap());
            } else {
                println!("fail {SELF}::Error: Failed to kill tmp server: {io_err}");
            }
        },
        Eval(ref req, ref target) => match req.send_and_recv(target) {
            Err(io_err) => println!("fail {SELF}::Error: Failed to send lua chunck: {io_err}"),
            Ok(ret_vals) => {
                if let Request::Return(vals) = ret_vals {
                    for val in vals {
                        print!("{val}");
                    }
                }
            }
        },
        WrongArgs => println!("fail {SELF}::Error: Wrong option"),
    }
}
