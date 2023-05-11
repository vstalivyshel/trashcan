use crate::SELF;
use std::path::PathBuf;
use std::{ffi::CString, io, os::unix::ffi::OsStrExt, path::Path};

#[macro_export]
macro_rules! f {
    ($($a:expr)*) => {{
        let mut nice = String::new();
        $(
        nice.push_str($a.as_ref());
        nice.push(' ');
        )*
        nice
    }}
}

pub fn print_info<S: std::fmt::Display>(msg: S) {
    println!("echo -debug {SELF}::Info: {msg}");
    println!("echo -markup {{Information}}{SELF}::Info: {msg}");
}

pub fn find_in(dir: PathBuf, target: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut found: Vec<PathBuf> = Vec::new();
    for entry in dir.read_dir()? {
        let path = entry?.path();
        if path.to_str().unwrap().contains(target) {
            found.push(path.to_path_buf());
        }
    }

    Ok(found)
}

pub fn temp_fifo_in<P: AsRef<Path>>(path: P) -> Option<tempfile::TempPath> {
    let temp_path = tempfile::TempPath::from_path(
        path.as_ref()
            .join(format!("{SELF}_temp_fifo_{:?}", rand::random::<u64>())),
    );

    if let Err(_) = create_fifo(&temp_path, 0o777) {
        None
    } else {
        Some(temp_path)
    }
}

pub fn create_fifo<P: AsRef<Path>>(path: &P, mode: libc::mode_t) -> Result<(), io::Error> {
    let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
    let fine = unsafe { libc::mkfifo(path.as_bytes_with_nul().as_ptr() as *const _, mode) == 0 };

    if fine {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn encode(msg: &str) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(msg.len() + 9);
    result.splice(..0, (msg.len() as u32).to_ne_bytes());
    msg.bytes().for_each(|b| result.push(b));
    result.splice(..0, (result.len() as u32 + 5).to_ne_bytes());
    result.insert(0, b'\x02');

    result
}

pub trait StringExt: AsRef<str> + Sized {
    fn sur_with(self, op: &str, cl: &str) -> String {
        let mut val = String::new();
        val.push_str(op);
        val.push_str(self.as_ref());
        val.push_str(cl);

        val
    }

    fn and<S: StringExt>(self, more: S) -> String {
        let mut new = String::new();
        new.push_str(self.as_ref());
        new.push_str(more.as_ref());

        new
    }

    fn for_sh(self) -> String {
        self.sur_with("kak_", "")
    }

    fn as_arg(self) -> String {
        self.sur_with("%arg¿", "¿")
    }

    fn as_reg(self) -> String {
        self.sur_with("%reg£", "£")
    }

    fn as_val(self) -> String {
        self.sur_with("%val®", "®")
    }

    fn as_opt(self) -> String {
        self.sur_with("%opt¶", "¶")
    }

    fn as_file(self) -> String {
        self.sur_with("%file{", "}")
    }

    fn sh(self) -> String {
        self.sur_with("%sh{", "}")
    }

    fn qt(self) -> String {
        self.sur_with("'", "'")
    }

    fn dqt(self) -> String {
        self.sur_with("\"", "\"")
    }

    fn kakqt(self) -> String {
        self.sur_with("%[", "]")
    }

    fn catch_err<S: StringExt>(self, err_cmd: S) -> String {
        try_catch(self.as_ref(), err_cmd.as_ref())
    }

    fn and_kakqt<S: StringExt>(self, cmd: S) -> String {
        self.and(" ").and(cmd.kakqt())
    }

    fn and_sh<S: StringExtChain>(self, args: S) -> String {
        self.and(" ").and(args.as_sh())
    }

    fn block<S: StringExtChain>(self, block: S) -> String {
        self.and(" ").and(block.kakqt())
    }
}

impl StringExt for String {}
impl StringExt for &String {}
impl StringExt for &str {}

pub trait StringExtChain: IntoIterator {
    fn as_cmd(self) -> String;
    fn as_eval(self) -> String;
    fn as_sh(self) -> String;
    fn kakqt(self) -> String;
}

impl<T: StringExt, I: IntoIterator<Item = T>> StringExtChain for I {
    fn as_cmd(self) -> String {
        let mut cmd = String::new();
        for c in self.into_iter() {
            cmd.push_str(c.as_ref());
            cmd.push_str("; ");
        }

        cmd
    }

    fn as_sh(self) -> String {
        let mut sh_cmd = String::new();
        for c in self.into_iter() {
            sh_cmd.push_str(c.as_ref());
            sh_cmd.push(' ');
        }

        sh_cmd.sh()
    }

    fn kakqt(self) -> String {
        self.as_cmd().kakqt()
    }

    fn as_eval(self) -> String {
        self.as_cmd().kakqt().sur_with("eval ", "")
    }
}

pub fn try_catch<S: StringExt>(try_cmd: S, catch_cmd: S) -> String {
    f!("try".and_kakqt(try_cmd) "catch".and_kakqt(catch_cmd))
}
