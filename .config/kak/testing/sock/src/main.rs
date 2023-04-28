mod utils;
mod kak_jsonrpc;
mod lua;
mod traits;
use kak_jsonrpc::IncomingRequest;
use log::{debug, error, info};
use lua::lua_exec;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::UnixStream;
use std::process::{self, Command, Stdio};
use traits::*;
use utils::{encode, send_to_kak_socket, conncet_to_kak_socket};

struct GluaClient {
    socket: UnixStream,
    stdin: BufWriter<process::ChildStdin>,
    stdout: BufReader<process::ChildStdout>,
    output_buffer: Vec<u8>,
    stderr: Option<BufReader<process::ChildStderr>>,
}

impl GluaClient {
    fn connect(session: &str) -> Result<Self, io::Error> {
        let mut process = Command::new("kak")
            .args([
                "-c",
                session,
                "-ui",
                "json",
                "-e",
                "rename-client GLUA; e -scratch *glua-debug*",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        let socket = conncet_to_kak_socket(session)?;
        let stdin = BufWriter::new(process.stdin.take().unwrap());
        let stdout = BufReader::new(process.stdout.take().unwrap());
        let output_buffer = Vec::<u8>::new();
        let stderr = Some(BufReader::new(process.stderr.take().unwrap()));

        Ok(Self {
            socket,
            stdin,
            stdout,
            output_buffer,
            stderr,
        })
    }

    fn send_to_socket(&mut self, msg: &str) -> Result<bool, io::Error> {
        send_to_kak_socket("sock", msg)
    }

    fn writeln_kakbuf(&mut self, buffer: &str, msg: &str) -> Result<(), io::Error> {
        let cmd = format! { "
            eval -save-regs 'g' -client GLUA %[
                try %[e -existing {buffer} ] catch %[e -scratch {buffer} ];
                set-register 'g' \"{msg}\" ;
                exec 'ge\"gP<a-o>' ;
            ]"
        };
        self.send_to_socket(&cmd)?;

        Ok(())
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
                        let title = title.collect_content();
                        let content = content.collect_content();

                        info!("InfoShow.title.content: \"{title}\"");
                        info!("InfoShow.content.content: \"{content}\"");
                        debug!("InfoShow: \n{request:?}");

                        if title.starts_with("EVAL") {
                            // Msg must not be empty!
                            match server.send_to_socket(&content) {
                                Err(io_err) =>  {
                                    error!("InfoShow::EVAL {io_err}");
                                }
                                Ok(is_written) => info!("InfoShow::Eval All msg written? {is_written}")
                            }
                        } else if title.starts_with("LUA") {
                            if let Err(lua_err) = lua_exec(content) {
                                error!("InfoShow::LUA: {lua_err}");
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
