mod kak_json;
mod utils;
mod traits;
use traits::*;
use kak_json::{IncomingRequest, OutgoingRequest, RawOutgoingRequest};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::process::{Command, Stdio};

// TODO: Test sending json request
// TODO: Try sending lua coed and eval it

fn spawn_json_client() -> std::process::Child {
    Command::new("kak")
        .args(["-c", "sock", "-ui", "json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
}

fn send_json_request<T: Write>(writer: &mut T, req: &OutgoingRequest) -> std::io::Result<String> {
    let request = serde_json::to_string::<OutgoingRequest>(req)?;
    writer.write_all(request.as_bytes())?;
    writer.flush()?;
    Ok(request)
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

fn get_some_json() {
    let mut server = spawn_json_client();

    let mut json_client_stdout = BufReader::new(server.stdout.take().expect("server stdout"));
    let mut json_client_stdin = BufWriter::new(server.stdin.take().expect("server stdin"));

    std::thread::spawn(move || {
        let json_client_stderr = BufReader::new(server.stderr.take().expect("server stderr"));
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
            }
        }
        println!("------------------------------------------------------------");
    }
}

fn send_to_socket(session_name: &str, msg: &str) {
    use std::os::unix::net::UnixStream;
    let rntimedir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let socket_path = std::path::Path::new(&rntimedir).join("kakoune").join(session_name);
    let mut stream = UnixStream::connect(socket_path).unwrap();

	let mut content = msg.bytes().collect::<Vec<u8>>();
	let encoded_cmd_len = (msg.len() as u32).to_ne_bytes();
	content.splice(..0, encoded_cmd_len);

	let header_byte = b'\x02';
	let encoded_whole_msg_len = (content.len() as u32 + 5).to_ne_bytes();
	content.splice(..0, encoded_whole_msg_len);
	content.insert(0, header_byte);

	// println!("{:?}" , String::from_utf8(content).unwrap());

    stream.write(&content).unwrap();
    stream.flush().unwrap();
}

fn main() {
    send_to_socket("sock", "");
}
