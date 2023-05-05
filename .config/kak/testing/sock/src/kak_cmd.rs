use crate::SELF;
use std::ffi::OsStr;
use std::path::Path;

pub const MAIN_COMMAND: &str = "glua-eval";
pub const VAL_SEP: &str = "§";
pub const EVACL: &str = "evaluate-commands -client";

pub enum Prefix {
    Opt,
    Val,
    Arg,
    Reg,
}

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

pub fn throw_error<B: Cmd, C: Cmd>(fail_msg: B, error: C) -> String {
    [
        "echo -markup {Error}".and_kakqt(fail_msg),
        "echo -debug".and_kakqt(SELF.and("::ERR\n").and(error.dqt())),
    ]
    .as_cmd()
}

pub fn try_catch<S: Cmd>(try_cmd: S, catch_cmd: S) -> String {
    f!("try".and_kakqt(try_cmd) "catch".and_kakqt(catch_cmd))
}

pub trait Cmd: AsRef<str> + Sized {
    fn sur_with(self, op: &str, cl: &str) -> String {
        let mut val = String::new();
        val.push_str(op);
        val.push_str(self.as_ref());
        val.push_str(cl);

        val
    }

    fn and<S: Cmd>(self, more: S) -> String {
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

    fn qt(self) -> String {
        self.sur_with("'", "'")
    }

    fn dqt(self) -> String {
        self.sur_with("\"", "\"")
    }

    fn kakqt(self) -> String {
        self.sur_with("%[", "]")
    }

    fn catch_err<S: Cmd>(self, err_cmd: S) -> String {
        try_catch(self.as_ref(), err_cmd.as_ref())
    }

    fn and_kakqt<S: Cmd>(self, cmd: S) -> String {
        self.and(" ").and(cmd.kakqt())
    }

    fn block<S: CmdChain>(self, block: S) -> String {
        self.and(" ").and(block.kakqt())
    }
}

impl Cmd for String {}
impl Cmd for &String {}
impl Cmd for &str {}

pub trait CmdChain: IntoIterator {
    fn as_cmd(self) -> String;
    fn as_eval(self) -> String;
    fn kakqt(self) -> String;
}

impl<T: Cmd, I: IntoIterator<Item = T>> CmdChain for I {
    fn as_cmd(self) -> String {
        let mut cmd = String::new();
        for c in self.into_iter() {
            cmd.push_str(c.as_ref());
            cmd.push_str("; ");
        }

        cmd
    }

    fn kakqt(self) -> String {
        self.as_cmd().kakqt()
    }

    fn as_eval(self) -> String {
        self.as_cmd().kakqt().sur_with("eval ", "")
    }
}

// fn main() {
//     // println!("{}", throw_error("client0", "this is err msg"));
//     // println!("{}", session_prelude());
//     // println!("{}", throw_error("client", "errmsg", "context"));
//     println!("{}", request_values(f!("suka".as_val() "blyad".as_opt())));
// }
