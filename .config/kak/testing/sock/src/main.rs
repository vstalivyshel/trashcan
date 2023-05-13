mod kakoune;
mod lua;
mod utils;
use kakoune::*;
use lua::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    env, io, fs,
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
};
use utils::*;

pub const SELF: &str = "GLUA";
pub const SOCK_HANDLER: &str = "GLUA_socket_root";

const SOCKET: &str = "GLUA.socket";
const ROOT: &str = "GLUA.root";
const PID_FILE: &str = "GLUA.pid";

enum Do {
    Kill(PathBuf),
    Spawn(PathBuf),
    Eval(Request, PathBuf),
    WrongArgs(String),
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
		let get_root = |path: String| -> Option<PathBuf> {
    		if path.is_empty() {
        		return None;
    		}

            let root = Path::new(&path);

            if root.is_dir() {
                return if root.to_str().unwrap().contains(ROOT) {
                    Some(root.canonicalize().unwrap())
                } else {
                    Some(root.canonicalize().unwrap().join(ROOT))
                }
            }

			let parent = root.parent();
            if parent.is_some() {
                let parent = parent.unwrap();
                let name = root.file_name().unwrap();
                if parent.to_str().unwrap().is_empty() {
                    let cwd = env::current_dir().unwrap();
                    return Some(cwd.join(name).with_extension(ROOT));
                } else {
                    let parent = Path::new(parent);
                    if parent.is_dir() {
                        return Some(parent.canonicalize().unwrap().join(name).with_extension(ROOT));
                    } else {
                        return None
                    }
                }
            }

            None
		};

        let arg_len = args.len();

        if arg_len < 1 {
            return WrongArgs("you need to specify a subcommand".into());
        }

        let sub = args.pop_front().unwrap();
        let sub = sub.as_str();

		let mut root = env::temp_dir().join(ROOT);

		if let Some(path) = args.pop_front() {
    		if let Some(p) = get_root(path) {
        		root = p
    		}
		}


        use Do::*;
        match sub {
            "spawn" => Spawn(root),
            "kill" =>  Kill(root),
            "eval" if arg_len > 4 => {
                let socket = root.join(SOCKET);
                match socket.try_exists() {
                    Err(e) => return WrongArgs(format!("can't check existence of {root}: {e}", root = root.display())),
                    Ok(exists) if !exists => return WrongArgs(format!("{socket:?} is invalid socket path")),
                    Ok(_) => {}
                }

                let this = ClientData {
                    session: args.pop_front().unwrap(),
                    client: args.pop_front().unwrap(),
                    chunck: args.pop_back().unwrap(),
                    chunck_args: args.into_iter().collect::<Vec<String>>(),
                };

                Eval(Request::LuaExec(this), socket)
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

    fn try_send_err<E: ToString>(&self, msg: &str, error: &E) -> Result<(), io::Error> {
        if let Request::LuaExec(ref d) = self {
            let session = &d.session;
            let client = &d.client;
            kak_throw_error(session, client, msg, &error.to_string())?;
        }

        Ok(())
    }
}

struct GluaServer {
    lua: Lua,
    root_path: PathBuf,
    pid_file: PathBuf,
    socket: UnixListener,
}

impl Drop for GluaServer {
    fn drop(&mut self) {
        if self.root_path.is_dir() {
             let _ = fs::remove_file(&self.root_path.join(SOCKET));
             let _ = fs::remove_file(&self.root_path.join(PID_FILE));
             let _ = fs::remove_dir(&self.root_path);
        }
    }
}

impl GluaServer {
    fn setup<P: AsRef<Path>>(root: P) -> Result<Self, mlua::Error> {
        let root_path = Path::new(root.as_ref()).to_path_buf();
        fs::DirBuilder::new().create(&root_path)?;
        let pid_file = root_path.join(PID_FILE);
        let socket = UnixListener::bind(&root_path.join(SOCKET))?;
        let lua = Lua::new();
        lua.prelude(root_path.to_str().unwrap().to_string())?;

        Ok(GluaServer {
            lua,
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
        Spawn(ref root) => match GluaServer::setup(root) {
            Err(io_err) => println!("fail {SELF}::Error: Failed to spawn server in {root}: {io_err}", root = root.display()),
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
        Kill(root) => match Request::Stop.send_to(root.join(SOCKET)) {
            Ok(()) => print_info(format!(
                "Killed: {sock}",
                 sock = root.display()
            )),
            Err(io_err) => println!(
                "fail {SELF}::Error: Failed to kill {sock}: {io_err}",
                sock = root.display(),
            ),
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
        WrongArgs(msg) => println!("fail {SELF}::Error: {msg}"),
    }
}
