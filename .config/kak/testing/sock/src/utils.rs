use std::os::unix::net::UnixStream;
use std::io::{Write};
use std::os::unix::fs::DirBuilderExt;
use std::path::PathBuf;

pub fn msg_to_socket(session_name: &str, msg: &str) -> std::io::Result<()> {
    let rntimedir = std::env::var("XDG_RUNTIME_DIR").expect("runtime path");
    let socket_path = std::path::Path::new(&rntimedir)
        .join("kakoune")
        .join(session_name);
    let mut stream = UnixStream::connect(socket_path)?;

    let mut result = Vec::<u8>::with_capacity(msg.len() + 9);
    result.splice(..0, (msg.len() as u32).to_ne_bytes());
    msg.bytes().for_each(|b| result.push(b));
    result.splice(..0, (result.len() as u32 + 5).to_ne_bytes());
    result.insert(0, b'\x02');
    stream.write(&result)?;
    stream.flush()?;

    Ok(())
}

pub fn glua_temp_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("glua-kak");
    std::fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&path)
        .unwrap();

    path
}

pub struct TempFifo {
    pub path: String,
}

impl Drop for TempFifo {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

pub fn temp_fifo(file_path: PathBuf) -> Option<TempFifo> {
    let path = file_path.to_str().unwrap().to_string();
    let p = std::ffi::CString::new(path.clone()).unwrap();

    let fifo_result = unsafe { libc::mkfifo(p.as_ptr(), 0o600) };

    if fifo_result != 0 {
        return None;
    }

    Some(TempFifo { path })
}

macro_rules! _kak_var {
    ($suf:expr, $op_ch:expr, $cl_ch:expr => $($a:expr)*) => {{
		let mut vals = String::new();
		$(
    		vals.push('%');
    		vals.push_str($suf);
    		vals.push($op_ch);
    		vals.push_str($a);
    		vals.push($cl_ch);
    		vals.push(' ');
		)*
		vals
    }}
}

macro_rules! _arg {
    ($($a:expr)*) => {
        kak_var!("arg", '¿', '¿' => $($a)*)
    };
}

macro_rules! _reg {
    ($($a:expr)*) => {
        kak_var!("reg", '£', '£' => $($a)*)
    };
}

macro_rules! _val {
    ($($a:expr)*) => {
        kak_var!("val", '§', '§' => $($a)*)
    };
}

macro_rules! _opt {
    ($($a:expr)*) => {
        kak_var!("opt", '¶', '¶' => $($a)*)
    };
}

macro_rules! _cmd {
    ($($($a:expr)*);*) => {{
        let mut cmd = String::new();
        $(
            $( cmd.push_str(&$a); cmd.push(' '); )*
            cmd.push_str("; ");
        )*
        cmd
    }};
}
