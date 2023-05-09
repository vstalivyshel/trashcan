mod kakoune;
mod utils;
use utils::*;
use kakoune::*;
use std::collections::VecDeque;
use std::process;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::os::unix::net::{UnixStream, UnixListener};
use tempfile::TempDir;

const MAIN_CMD: &str = "glua-eval-test";

struct Server {
    pid_file: PathBuf,
    root: TempDir,
    socket: UnixListener,
}

impl Server {
    fn setup(session: &str) -> Result<Self, io::Error>{
        let root = tempfile::Builder::new().prefix(&format!("{SELF}_{session}")).tempdir()?;
        let root_path = root.path();
        let pid_file = root_path.join(&format!("{SELF}_{session}.pid"));
        let socket_path = root_path.join(&format!("{SELF}_{session}.socket"));
        let socket = UnixListener::bind(&root_path.join(&socket_path))?;

        Ok(Server {
            pid_file,
            root,
            socket,
        })
    }

    fn run(self) -> Result<(), io::Error> {
        let mut buffer = String::new();
        let mut socket = self.socket.incoming();
        for stream in socket {
            let mut stream = stream.unwrap();
            stream.read_to_string(&mut buffer)?;
            kak_send_msg()
            if buffer.contains("stop") {
                break;
            }
            buffer.clear();
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Request {
    session: String,
    client: String,
    chunck: Vec<u8>,
    chunck_args: Vec<String>,
}

fn main() {
    let mut args = std::env::args().skip(1).collect::<VecDeque<String>>();
    let self_cmd = std::env::current_exe().unwrap();
    let self_cmd = self_cmd.to_str().unwrap();

    if args.len() < 4 {
        let sub_cmd = args.get(0);
        if sub_cmd.is_none() || !sub_cmd.unwrap().starts_with("init") {
            println!("fail {SELF}::Error: Your {SELF} command have wrong argument count!");
            process::exit(69);
        }

        let socket_handler = SELF.and("_socket_handler");
        let init_cmd = [
            f!("declare-option -hidden str" socket_handler),
            f!("set-option global" socket_handler "ready".qt()),
            f!("define-command" MAIN_CMD "-override -params 1..").and_kakqt(
                "evaluate-commands".and_sh([
                    self_cmd.to_string(),
                    "$kak_opt_".and(socket_handler).dqt(),
                    "$kak_session".dqt(),
                    "$kak_client".dqt(),
                    "$@".dqt(),
                ]),
            ),
            f!("alias global lua" MAIN_CMD),
        ].as_cmd();

        println!("{init_cmd}");
        process::exit(0);
    }

    let socket = args.pop_front().unwrap();
    let session = args.pop_front().unwrap();
    let client = args.pop_front().unwrap();
    let chunck = args.pop_back().unwrap().into_bytes();
    let chunck_args = args.into_iter().collect::<Vec<String>>();

    // let data = Request {
    //     session,
    //     client,
    //     chunck,
    //     chunck_args,
    // };

    if socket.starts_with("ready") {
        let server = Server::setup(&session).unwrap();
        let cwd = std::env::current_dir().unwrap();
        let daemon = daemonize::Daemonize::new()
            .pid_file(&server.pid_file)
            .working_directory(&cwd)
            .start()
            .unwrap();
    } else {
        let mut stream = UnixStream::connect(&socket).unwrap();
        stream.write_all(b"echo hello").unwrap();
    }
}
