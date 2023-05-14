mod lua;
mod utils;
use lua::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    env, fs, io,
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
};
use utils::*;

pub const SELF: &str = "GLUA";
const SOCK_EXT: &str = "glua_socket";

enum CliOpt {
    StgPush(Vec<String>, PathBuf),
    StgPop(PathBuf),
    Kill(PathBuf),
    Spawn(PathBuf),
    Eval(Request, PathBuf),
    WrongArgs(String),
}

#[derive(Serialize, Deserialize, Clone)]
enum Request {
    StgPop,
    StgPush(Vec<String>),
    LuaExec(ClientData),
    Return(Vec<String>),
    Continue,
    Stop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    session: String,
    client: String,
    chunk: String,
    chunk_args: Vec<String>,
}

impl CliOpt {
    fn from_args() -> Self {
        let get_root = |path: &String| -> Option<PathBuf> {
            if path.is_empty() {
                return None;
            }

            let root = Path::new(&path);

            if root.exists() {
                return Some(root.canonicalize().unwrap());
            }

            if let Some(parent) = root.parent() {
                let name = root.file_name().unwrap();
                if parent.to_str().unwrap().is_empty() {
                    let cwd = env::current_dir().unwrap();
                    return Some(cwd.join(name).with_extension(SOCK_EXT));
                } else {
                    let parent = Path::new(parent);
                    if parent.is_dir() {
                        return Some(
                            parent
                                .canonicalize()
                                .unwrap()
                                .join(name)
                                .with_extension(SOCK_EXT),
                        );
                    } else {
                        return None;
                    }
                }
            }

            None
        };
        let mut args = std::env::args().skip(1);
        let sub = args.next();

        if sub.is_none() {
            return WrongArgs("you need to specify a subcommand".into());
        }

        let sub = sub.unwrap();

        let mut root = env::temp_dir().join(SOCK_EXT);
        if let Some(path) = args.next() {
            if let Some(p) = get_root(&path) {
                root = p
            }
        }

        use CliOpt::*;
        match sub.as_str() {
            "push" => StgPush(args.collect::<Vec<String>>(), root),
            "pop" => StgPop(root),
            "spawn" => Spawn(root),
            "kill" => Kill(root),
            "eval" => {
                let mut args = args.collect::<VecDeque<String>>();
                if args.len() < 3 {
                    return WrongArgs("wrong argument count".into());
                }
                match root.try_exists() {
                    Err(e) => {
                        return WrongArgs(format!(
                            "can't check existence of {root}: {e}",
                            root = root.display()
                        ))
                    }
                    Ok(exists) if !exists => {
                        return WrongArgs(format!("{root:?} is invalid socket path"))
                    }
                    Ok(_) => {}
                }

                let this = ClientData {
                    session: args.pop_front().unwrap(),
                    client: args.pop_front().unwrap(),
                    chunk: args.pop_back().unwrap(),
                    chunk_args: args.into_iter().collect::<Vec<String>>(),
                };

                Eval(Request::LuaExec(this), root)
            }
            _ => WrongArgs("wrong argument count".into()),
        }
    }
}

impl Request {
    fn send_to<P: AsRef<Path>>(&self, socket: P) -> Result<(), bincode::Error> {
        bincode::serialize_into(UnixStream::connect(socket)?, self)
    }

    fn send_and_recv<P: AsRef<Path>>(&self, socket: P) -> Result<Request, bincode::Error> {
        let stream = UnixStream::connect(socket)?;
        bincode::serialize_into(&stream, self)?;
        bincode::deserialize_from::<_, Request>(&stream)
    }

    fn send_back(&self, stream: &UnixStream) -> Result<(), bincode::Error> {
        bincode::serialize_into(stream, self)
    }

    fn try_send_err<E: ToString>(&self, msg: &str, error: &E) -> Result<(), io::Error> {
        if let Request::LuaExec(ref d) = self {
            let session = &d.session;
            let client = &d.client;
            kak_throw_error(session, client, msg, error.to_string())?;
        }

        Ok(())
    }
}

struct GluaServer {
    lua: Lua,
    root: TempFile,
    socket: UnixListener,
}

impl GluaServer {
    fn setup<P: AsRef<Path>>(path: P) -> Result<Self, mlua::Error> {
        let root = TempFile::from(path);
        let root_path = &root.path;
        if root_path.exists() {
            let _ = Request::Stop.send_to(root_path);
            let _ = fs::remove_file(root_path);
        }
        let socket = UnixListener::bind(root_path)?;
        let lua = Lua::new();
        lua.prelude(root_path.to_str().unwrap())?;

        Ok(GluaServer { lua, root, socket })
    }

    fn run(self) {
        let mut last_request = Request::Continue;
        let mut storage = Vec::<Vec<String>>::new();
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
                        let req = if ret_vals.is_empty() {
                            Continue
                        } else {
                            Return(ret_vals)
                        };

                        if let Err(de_err) = req.send_back(&stream) {
                            let _ = last_request
                                .try_send_err("Failed to receive return values from lua", &de_err);
                        }
                    }
                    Err(lua_err) => {
                        let _ = last_request.try_send_err("Lua error", &lua_err);
                    }
                },
                StgPop => {
                    let req = if let Some(saved) = storage.pop() {
                        Return(saved)
                    } else {
                        Continue
                    };

                    if let Err(e) = req.send_back(&stream) {
                        let _ =
                            last_request.try_send_err("Failed to return values from storage", &e);
                    }
                }
                StgPush(data) => storage.push(data),
                Continue => continue,
                Stop => break,
                Return(_) => unreachable!("what?"),
            }
        }
    }
}

fn start() {
    use CliOpt::*;
    match CliOpt::from_args() {
        Spawn(ref root) => match GluaServer::setup(root) {
            Err(io_err) => println!(
                "fail {SELF}::Error: Failed to spawn server in {root}: {io_err}",
                root = root.display()
            ),
            Ok(server) => {
                let socket_root = server.root.path.to_str().unwrap();
                print_info(f!("Born in" socket_root.dqt()));

                if let Err(d_err) = daemonize::Daemonize::new()
                    .working_directory(env::current_dir().unwrap())
                    .start()
                {
                    println!("fail {SELF}::Error: Failed to daemonize: {d_err}");
                } else {
                    server.run();
                }
            }
        },
        Kill(ref root) => match Request::Stop.send_to(root) {
            Ok(()) => print_info(format!("Killed: {sock}", sock = root.display())),
            Err(io_err) => println!(
                "fail {SELF}::Error: Failed to kill {sock}: {io_err}",
                sock = root.display(),
            ),
        },
        StgPush(args, socket) => match Request::StgPush(args).send_to(socket) {
            Err(e) => println!("fail {SELF}::Error: Failed to push data into storage: {e}"),
            Ok(_) => print_info("Data saved"),
        },
        StgPop(socket) => match Request::StgPop.send_and_recv(&socket) {
            Err(e) => println!(
                "fail {SELF}::Error: Failed to get data from {s}: {e}",
                s = socket.display()
            ),
            Ok(data) => {
                if let Request::Return(vals) = data {
                    for val in vals {
                        println!("{val}");
                    }
                }
            }
        },
        Eval(req, socket) => match req.send_and_recv(socket) {
            Err(io_err) => println!("fail {SELF}::Error: Failed to send lua chunck: {io_err}"),
            Ok(ret_vals) => {
                if let Request::Return(vals) = ret_vals {
                    for val in vals {
                        println!("{val}");
                    }
                }
            }
        },
        WrongArgs(msg) => println!("fail {SELF}::Error: {msg}"),
    }
}

fn main() {
    start();
}
