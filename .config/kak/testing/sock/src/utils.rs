use std::io::{self, Write};
use std::os::unix::net::UnixStream;

pub fn send_to_kak_socket(session: &str, msg: &str) -> Result<bool, io::Error> {
    let mut stream = conncet_to_kak_socket(&session)?;
    let written_amount = stream.write(&encode(msg))?;
    stream.flush()?;

    Ok(written_amount == msg.len())
}

pub fn conncet_to_kak_socket(session: &str) -> Result<UnixStream, io::Error> {
    // TODO: Need to handle this unwrap
    let rntimedir = std::env::var("XDG_RUNTIME_DIR").expect("runtime path");
    let socket_path = std::path::Path::new(&rntimedir)
        .join("kakoune")
        .join(session);

    UnixStream::connect(socket_path)
}

pub fn encode(msg: &str) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(msg.len() + 9);
    result.splice(..0, (msg.len() as u32).to_ne_bytes());
    msg.bytes().for_each(|b| result.push(b));
    result.splice(..0, (result.len() as u32 + 5).to_ne_bytes());
    result.insert(0, b'\x02');

    result
}

macro_rules! kak_var {
    ($suf:expr, $op_ch:expr, $cl_ch:expr => $($var_name:expr)*) => {{
		let mut vals = String::new();
		$(
    		vals.push('%');
    		vals.push_str($suf);
    		vals.push($op_ch);
    		vals.push_str($var_name);
    		vals.push($cl_ch);
    		vals.push(' ');
		)*
		vals
    }}
}

#[macro_export]
macro_rules! arg {
    ($($a:expr)*) => {
        kak_var!("arg", '¿', '¿' => $($a)*)
    };
}

#[macro_export]
macro_rules! reg {
    ($($a:expr)*) => {
        kak_var!("reg", '£', '£' => $($a)*)
    };
}

#[macro_export]
macro_rules! val {
    ($($a:expr)*) => {
        kak_var!("val", '§', '§' => $($a)*)
    };
}

#[macro_export]
macro_rules! opt {
    ($($a:expr)*) => {
        kak_var!("opt", '¶', '¶' => $($a)*)
    };
}

macro_rules! cmd {
    ($($($a:expr)*);*) => {{
        let mut cmd = String::new();
        $(
            $( cmd.push_str(&$a); cmd.push(' '); )*
            cmd.push_str("; ");
        )*
        cmd
    }};
}
