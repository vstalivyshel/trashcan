#[macro_use]
mod utils;
mod kak_jsonrpc;
mod lua;
mod traits;
use kak_jsonrpc::{IncomingRequest, OutgoingRequest};
use log::{debug, error, info};
use lua::lua_exec;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process::{self, Command, Stdio};
use traits::*;
use utils::msg_to_socket;

struct JsonClient {
    session: String,
    process: process::Child,
    stdout: BufReader<process::ChildStdout>,
    stdin: BufWriter<process::ChildStdin>,
    stderr: Option<BufReader<process::ChildStderr>>,
    read_buffer: Vec<u8>,
}

impl JsonClient {
    fn connect(session_name: &str) -> Result<Self, io::Error> {
        let mut process = Command::new("kak")
            .args([
                "-c",
                session_name,
                "-ui",
                "json",
                "-e",
                "rename-client GLUA",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        let session = session_name.to_string();
        let stdout = BufReader::new(process.stdout.take().unwrap());
        let stdin = BufWriter::new(process.stdin.take().unwrap());
        let stderr = Some(BufReader::new(process.stderr.take().unwrap()));
        let read_buffer = Vec::<u8>::new();

        Ok(Self {
            session,
            process,
            stdout,
            stdin,
            stderr,
            read_buffer,
        })
    }

    fn send_to_socket(&self, msg: &str) -> Result<(), io::Error> {
        msg_to_socket(&self.session, msg)?;

        Ok(())
    }

    fn send_request(&mut self, out_request: &OutgoingRequest) -> Result<String, io::Error> {
        let req = serde_json::to_string::<OutgoingRequest>(out_request)?;
        self.stdin.write_all(req.as_bytes())?;
        self.stdin.flush()?;

        Ok(req)
    }

    fn read_request(&mut self) -> Result<IncomingRequest, serde_json::Error> {
        self.read_buffer.clear();
        self.stdout
            .read_until(b'\n', &mut self.read_buffer)
            .unwrap();
        serde_json::from_slice::<IncomingRequest>(&self.read_buffer)
    }
}

fn run() {
    let mut server = JsonClient::connect("sock").expect("glua server");

    // TODO: send error messages to the kakoune session
    let mut stderr = server.stderr.take().unwrap();
    std::thread::spawn(move || {
        let mut err_buf = Vec::<u8>::new();
        loop {
            if let Err(read_error) = stderr.read_until(b'\n', &mut err_buf) {
                error!("JsonClien: Failed to read stderr: {read_error}");
            } else {
                error!(
                    "JsonClien: Error while parsing outgoing json request: {err}",
                    err = std::str::from_utf8(&err_buf).unwrap()
                );
            }
            err_buf.clear();
        }
    });

	// TODO: create another thread that will handle events related for only for glua clients
	// 		 like getting values from session, clients list etc
    loop {
        match server.read_request() {
            Err(parse_err) => {
                error!("IncomingRequest: Error while parsing: {parse_err}");
                if parse_err.is_eof() {
                    break;
                } else {
                    continue;
                }
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
                        let title = title.collect_content();
                        let content = content.collect_content();

                        info!("InfoShow.title.content = \"{title}\"");
                        info!("InfoShow.content.content = \"{content}\"");
                        debug!("InfoShow.debug = \n{request:?}");

                        if title.starts_with("SOCK") {
                            if let Err(io_err) = server.send_to_socket(&content) {
                                error!("InfoShow::SOCK {io_err}");
                            }
                        } else if title.starts_with("LUA") {
                            lua_exec(content).unwrap();
                        } else if title.starts_with("EXEC") {
                            if let Err(io_err) = server.send_request(&OutgoingRequest::Keys(vec![content])) {
                                error!("InfoShow::EXEC {io_err}")
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
                        info!("status_line.content = \"{status_msg}\"");
                        info!("mode_line.content = \"{mode_line_content}\"");
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

