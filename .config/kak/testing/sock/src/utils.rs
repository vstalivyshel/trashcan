use regex::Regex;
use std::ffi::CString;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::UnixStream;
use std::path::Path;

pub fn create_fifo<P: AsRef<Path>>(path: &P, mode: libc::mode_t) -> io::Result<()> {
    let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
    let fine = unsafe { libc::mkfifo(path.as_bytes_with_nul().as_ptr() as *const _, mode) == 0 };

    if fine {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn send_to_kak_socket(session: &str, msg: &str) -> Result<(), io::Error> {
    let rntm = std::env::var("XDG_RUNTIME_DIR").expect("runtimedir");
    let socket = std::path::PathBuf::from(rntm).join("kakoune").join(session);
    let mut stream = UnixStream::connect(socket)?;
    stream.write(&encode(msg))?;
    stream.flush()?;

    Ok(())
}

pub fn encode(msg: &str) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(msg.len() + 9);
    result.splice(..0, (msg.len() as u32).to_ne_bytes());
    msg.bytes().for_each(|b| result.push(b));
    result.splice(..0, (result.len() as u32 + 5).to_ne_bytes());
    result.insert(0, b'\x02');

    result
}
