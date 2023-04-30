#[macro_use]
mod utils;
mod kak_jsonrpc;
mod lua;
mod traits;
mod kak_cmd;
use kak_jsonrpc::IncomingRequest;
use log::{debug, error, info};
use lua::lua_exec;
use std::fmt::Display;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::process::{self, Command, Stdio};
use traits::*;
use utils::send_to_kak_socket;

struct GluaClient {
    session: String,
    stdin: BufWriter<process::ChildStdin>,
    stdout: BufReader<process::ChildStdout>,
    output_buffer: Vec<u8>,
    stderr: Option<BufReader<process::ChildStderr>>,
}

// TODO: how to get values? options? registers? what is the best way?
// TODO: get client list as iterator => create function that runs cmd in context of evry client
impl GluaClient {
    fn connect(session: &str) -> Result<Self, io::Error> {
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

        send_to_kak_socket(session, kak_cmd::session_prelude())?;

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

    fn send_to_socket(&self, msg: &str) -> Result<(), io::Error> {
        send_to_kak_socket(&self.session, msg)
    }

    fn read_request(&mut self) -> Result<IncomingRequest, serde_json::Error> {
        self.stdout
            .read_until(b'\n', &mut self.output_buffer)
            .expect("read json output");
        let inc_req = serde_json::from_slice::<IncomingRequest>(&self.output_buffer);
        self.output_buffer.clear();

        inc_req
    }
}

fn run() {
    let mut server = GluaClient::connect("sock").expect("spawn json client");

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

// eval -client client0 %{ eval -client GLUA %{ exec ":echo %val{bufname}|%val{session}<a-!><ret>" } }
