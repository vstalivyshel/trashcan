mod kak_jsonrpc;
mod traits;
mod utils;
use utils::msg_to_socket;
use serde_json::{Result};
use kak_jsonrpc::{IncomingRequest, OutgoingRequest, RawOutgoingRequest};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process::{self, Command, Stdio};
use traits::*;

// TODO: Test sending json request
// TODO: Try sending lua coed and eval it

struct JsonClient {
    process: process::Child,
    stdout: BufReader<process::ChildStdout>,
    stderr: BufReader<process::ChildStderr>,
    stdin: BufWriter<process::ChildStdin>,
}

impl JsonClient {
    fn connect(session_name: &str) -> std::io::Result<Self> {
        let mut process = Command::new("kak")
            .args(["-c", "sock", "-ui", "json"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        let stdout = BufReader::new(process.stdout.take().unwrap());
        let stderr = BufReader::new(process.stderr.take().unwrap());
        let stdin = BufWriter::new(process.stdin.take().unwrap());

        Ok(Self {
            process,
            stdout,
            stderr,
            stdin,
        })
    }

    fn send_request(&mut self, out_request: &OutgoingRequest) -> std::io::Result<String> {
        let req = serde_json::to_string::<OutgoingRequest>(out_request)?;
        self.stdin.write_all(req.as_bytes())?;
        self.stdin.flush()?;

        Ok(req)
    }

}

fn process_incoming_request(method: IncomingRequest) {
    println!("GLUA::IncomingRequest = {method}");
    match method {
        IncomingRequest::InfoShow {
            ref title,
            ref content,
            ..
        } => {
            let info_content = content.get_content();
            let title_content = title.get_content();
            println!("GLUA::InfoShow.title.content = \"{title_content}\"");
            println!("GLUA::InfoShow.content.content = \"{info_content}\"");
        }
        IncomingRequest::DrawStatus {
            ref status_line,
            ref mode_line,
            ..
        } => {
            let status_msg = status_line.get_content();
            let mode_line_content = mode_line.get_content();
            println!("GLUA::status_line.content = \"{status_msg}\"");
            println!("GLUA::mode_line.content = \"{mode_line_content}\"");
        }

        _ => {}
    }
}

fn run() {
    let mut server = JsonClient::connect("sock").expect("glua server");

    let mut json_client_stdout = server.stdout;
    let mut json_client_stdin = server.stdin;

    std::thread::spawn(move || {
        let json_client_stderr = server.stderr;
        for err in json_client_stderr.lines() {
            println!(
                "GLUA::ERR = Error while parsing outgoing json request: {err}",
                err = err.unwrap()
            );
        }
    });
    let mut buffer = Vec::<u8>::new();
    loop {
        buffer.clear();
        json_client_stdout.read_until(b'\n', &mut buffer).unwrap();
        let incom_request = serde_json::from_slice::<IncomingRequest>(&buffer);
        match incom_request {
            Ok(method) => {
                process_incoming_request(method);
            }
            Err(error) => {
                println!(
                    "GLUA::ERR = Error while parsing incoming json request: {}",
                    error
                );
                if error.is_eof() {
                    break;
                }
            }
        }
        println!("------------------------------------------------------------");
    }
}

fn main() {
    run();
}
