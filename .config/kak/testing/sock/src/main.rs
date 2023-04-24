mod json_ui;
use json_ui::{IncomingRequest, OutgoingRequest};
use std::io::{BufWriter, BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};

fn spawn_json_client() -> std::process::Child {
    Command::new("kak")
        .args(["-c", "sock", "-ui", "json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap()
}


fn get_some_json() {
    let mut server = spawn_json_client();
    let output = server.stdout.take().unwrap();
    let inp = server.stdin.take().unwrap();
    let err_out = server.stderr.take().unwrap();
    let reader = BufReader::new(output);
    let mut writer = BufWriter::new(inp);
	let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut err_reader = BufReader::new(err_out);
		for line in err_reader.lines() {
    		sender.send(line.unwrap());
		}
    });

	let mut err = String::new();
    for line in reader.lines() {
        let method: IncomingRequest = serde_json::from_str(&line.unwrap()).unwrap();
        match method {
            IncomingRequest::InfoShow { .. } => {
                println!("{:?}", method);
                let out_method = OutgoingRequest::Keys(vec![ ":eval -client client0 %{ echo sasha }<ret>".to_string() ]);
                println!("{:?}", out_method);
                serde_json::to_writer(&mut writer, &out_method).unwrap();
                if let Ok(err_msg) = receiver.try_recv() {
                    println!("Stderr msg : {err_msg}");
                }
            },
            IncomingRequest::DrawStatus { status_line, mode_line, default_face, } => {
                let status_msg = status_line.iter().map(|atom| atom.contents)
            },

            _ => println!("{}", method),
        }
    }
}

fn main() {
    get_some_json();
}
