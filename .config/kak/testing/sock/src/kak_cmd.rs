pub const SELF: &str = "GLUA";
pub const MAIN_COMMAND: &str = "glua-eval";
pub const CLIENT_HANDLER: &str = "glua_current_client";
pub const VAL_HANDLER: &str = "glua_val_handler";
pub const VAL_SEP: &str = "§";
pub const CMD_DEL: &str = "; ";

pub const SET: &str = "set-option";
pub const SET_ADD: &str = "set-option -add";
pub const SET_REG: &str = "set-register";
pub const DEF: &str = "define-command";
pub const EVAL: &str = "evaluate-commands";
pub const EVACL: &str = "evaluate-commands -client";
pub const DECL: &str = "declare-option";
pub const INFO: &str = "info";
pub const EXEC: &str = "execute-keys";
pub const TRY_CATCH: [&str; 2] = ["try", "catch"];
pub const ECHO: &str = "echo";
pub const ECHO_DBG: &str = "echo -debug";
pub const FAIL: &str = "fail";

pub const GLOB: &str = "global";

pub const NOP: &str = "''";
pub const NIL: &str = "nil";

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

pub fn session_prelude() -> String {
    let decl = "declare-option -hidden str";
    [
        f!(decl VAL_HANDLER),
        f!(decl CLIENT_HANDLER),
        f!(DEF MAIN_COMMAND "-override -params 1..").block([
            f!(SET GLOB CLIENT_HANDLER "client".as_val()),
            f!(EVACL SELF).and_kakqt(
                f!(INFO "-title" CLIENT_HANDLER.as_opt() "@".as_arg().dqt())
            ),
        ]),
        f!("alias" GLOB "lua" MAIN_COMMAND),
    ]
    .as_cmd()
}

pub enum Prefix {
    Opt,
    Val,
    Arg,
    Reg,
}

pub fn request_values<A: Cmd, B: Cmd>(pref: Prefix, client: A, vars: B) -> String {
    f!(EVACL client).block([
        f!(SET GLOB VAL_HANDLER NOP),
        f!(SET_ADD GLOB VAL_HANDLER vars.dqt()),
        f!(EVACL SELF).and_kakqt(f!(INFO VAL_HANDLER.as_opt().dqt())),
    ])
}

pub fn extract_values(values: String) -> Vec<String> {
    values
        .split_terminator(VAL_SEP)
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
}

pub fn try_request_values<S, I>(vars: I) -> String
where
    S: Cmd,
    I: IntoIterator<Item = S>,
{
    [
        f!(SET GLOB VAL_HANDLER NOP),
        vars.into_iter()
            .map(|var| {
                try_catch(
                    f!(SET_ADD GLOB VAL_HANDLER var.and(VAL_SEP).dqt()),
                    f!(SET_ADD GLOB VAL_HANDLER NIL.and(VAL_SEP).qt()),
                )
                .and(CMD_DEL)
            })
            .collect::<String>(),
        f!(EVACL SELF).and(f!(INFO "-title VALS" VAL_HANDLER.as_opt().dqt())), // f!(MAIN_COMMAND VAL_HANDLER.as_opt().dqt()),
    ]
    .as_cmd()
}

pub fn eval_self<S: Cmd, I: IntoIterator<Item = S>>(cmds: I) -> String {
    f!(EVACL SELF cmds.kakqt())
}

pub fn throw_error<A: Cmd, B: Cmd, C: Cmd>(client: A, fail_msg: B, error: C) -> String {
    f!(EVACL client).block([
        f!(ECHO "-markup" "{Error}".and(fail_msg).kakqt()),
        f!(ECHO_DBG SELF.and("::ERR ").and(error.dqt()).kakqt()),
    ])
}

pub fn writeln_kakbuf<S: Cmd>(buffer: S, msg: S) -> String {
    let buffer = buffer.qt();
    [
        f!("edit -existing" buffer).catch_err(f!("edit -scratch" buffer)),
        f!(SET_REG "g".qt() msg.dqt()),
        f!(EXEC "gegh\"gP<a-o>".qt()),
    ]
    .as_cmd()
}

pub fn try_catch<S: Cmd>(try_cmd: S, catch_cmd: S) -> String {
    f!(TRY_CATCH[0] try_cmd.kakqt() TRY_CATCH[1] catch_cmd.kakqt())
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

pub trait CmdChain {
    fn as_cmd(self) -> String;
    fn as_eval(self) -> String;
    fn kakqt(self) -> String;
}

impl<T: Cmd, I: IntoIterator<Item = T>> CmdChain for I {
    fn as_cmd(self) -> String {
        let mut cmd = String::new();
        for c in self.into_iter() {
            cmd.push_str(c.as_ref());
            cmd.push_str(CMD_DEL);
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
