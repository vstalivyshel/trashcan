#[macro_use]
mod utils;
mod kak_cmd;
mod kak_jsonrpc;
mod lua;
mod traits;
use crate::lua::{LuaState, Lua, Operation};
use crate::kak_cmd::{SELF, Cmd};
use crate::kak_jsonrpc::IncomingRequest;
use crate::traits::*;
use crate::utils::send_to_kak_socket;
use log::{debug, error, info};
use std::fmt::Display;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::process::{self, Command, Stdio};

pub struct GluaClient {
    pub session: String,
    pub stdin: BufWriter<process::ChildStdin>,
    pub stdout: BufReader<process::ChildStdout>,
    pub output_buffer: Vec<u8>,
    pub stderr: Option<BufReader<process::ChildStderr>>,
}

// TODO: how to get values? options? registers? what is the best way?
// TODO: get client list as iterator => create function that runs cmd in context of evry client
impl GluaClient {
    pub fn connect(session: &str) -> Result<Self, io::Error> {
        let mut process = Command::new("kak")
            .args([
                "-c",
                session,
                "-ui",
                "json",
                "-e",
                "rename-client GLUA; e -scratch *scratch*",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        send_to_kak_socket(session, &kak_cmd::session_prelude())?;

        let session = session.to_string();
        let stdin = BufWriter::new(process.stdin.take().unwrap());
        let stdout = BufReader::new(process.stdout.take().unwrap());
        let output_buffer = Vec::<u8>::new();
        let stderr = Some(BufReader::new(process.stderr.take().unwrap()));

        Ok(Self {
            session,
            stdin,
            stdout,
            output_buffer,
            stderr,
        })
    }

    pub fn send_to_socket(&self, msg: &str) -> Result<(), io::Error> {
        send_to_kak_socket(&self.session, msg)
    }

    pub fn read_request(&mut self) -> Result<IncomingRequest, serde_json::Error> {
        self.stdout
            .read_until(b'\n', &mut self.output_buffer)
            .expect("read json output");
        let inc_req = serde_json::from_slice::<IncomingRequest>(&self.output_buffer);
        self.output_buffer.clear();

        inc_req
    }
}

fn run() {
    let session = match std::env::args().skip(1).next() {
        Some(s) => s,
        None => {
            eprintln!("{SELF}::Err: Session name is not specified");
            process::exit(69);
        },
    };
    let mut server = match GluaClient::connect(&session) {
        Ok(serv) => serv,
        Err(io_err) => {
            eprintln!("{SELF}::Err: Failed to spawn client: {io_err}");
            process::exit(69);
        }
    };

    let lua = match Lua::init_state() {
        Ok(l) => l,
        Err(lua_err) => {
            eprintln!("{SELF}::Err: Failed to init Lua State: {lua_err}");
            process::exit(69);
        }
    };

    loop {
        match server.read_request() {
            Err(parse_err) => {
                error!("IncomingRequest: Error while parsing: {parse_err}");
                if parse_err.is_eof() {
                    break;
                }
                continue;
            }
            Ok(request) => {
                info!("IncomingRequest = {request}");

                use IncomingRequest::*;
                match request {
                    InfoShow {
                        ref title,
                        ref content,
                        ..
                    } => {
                        let client = title.collect_content();
                        let chunk = content.collect_content();

                        info!("InfoShow.title.content: \"{client}\"");
                        info!("InfoShow.content.content: \"{chunk}\"");
                        debug!("InfoShow: \n{request:?}");

                        if chunk.is_empty() {
                            continue;
                        }

                        match lua.eval_and_get_ops(chunk) {
                            Ok(ops) => {
                                for op in ops.iter() {
                                    use Operation::*;
                                    match op {
                                        RawEval(cmd) => log::info!("LUA: Requested raw evaluation: {cmd}"),
                                        ToSocket(ses_name, cmd) => log::info!("LUA: Req to socket {ses_name} cmd: {cmd}"),
                                    }
                                }
                            }
                            Err(lua_err) => {
                                server.send_to_socket(&kak_cmd::throw_error(
                                    &client,
                                    "Err executing lua chunk! See debug",
                                    lua_err.to_string(),
                                )).expect("send lua error to socket");
                            }
                        }
                    }
                    DrawStatus {
                        ref status_line,
                        ref mode_line,
                        ..
                    } => {
                        let status_msg = status_line.collect_content();
                        let mode_line_content = mode_line.collect_content();
                        info!("DrawStatus.status_line.content = \"{status_msg}\"");
                        info!("DrawStatus.mode_line.content = \"{mode_line_content}\"");
                    }

                    _ => {}
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    run();
}
