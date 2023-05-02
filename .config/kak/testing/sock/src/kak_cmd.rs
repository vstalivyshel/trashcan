pub const SELF: &str = "GLUA";
pub const MAIN_COMMAND: &str = "glua-eval";
pub const CLIENT_HANDLER: &str = "glua_current_client";
pub const VAL_HANDLER: &str = "glua_val_handler";
pub const VAL_HANDLER_DEL: &str = "§";

pub const CMD_DEL: &str = "; ";

pub const SET: &str = "set-option";
pub const SET_ADD: &str = "set-option -add";
pub const SET_REG: &str = "set-register";
pub const DEF: &str = "define-command";
pub const EVAL: &str = "evaluate-commands";
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
macro_rules! fmt {
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
    [
        fmt!(DECL "-hidden str" VAL_HANDLER),
        fmt!(DECL "-hidden str" CLIENT_HANDLER),
        cmd(
            fmt!(DEF MAIN_COMMAND "-override -params .."),
            [
                fmt!(SET GLOB CLIENT_HANDLER "client".as_val()),
                cmd(
                    fmt!(EVAL "-client" SELF),
                    fmt!(INFO "-title" CLIENT_HANDLER.as_opt() "@".as_arg().dqt()),
                ),
            ]
            .join(CMD_DEL),
        ),
        fmt!("alias" GLOB "lua" MAIN_COMMAND),
    ]
    .join(CMD_DEL)
}

pub fn set_val_handler<S, I>(vars: I) -> String
where
    S: Cmd,
    I: IntoIterator<Item = S>,
{
    [
        fmt!(SET GLOB VAL_HANDLER NOP),
        vars.into_iter()
            .map(|var| {
                try_catch(
                    fmt!(SET_ADD GLOB VAL_HANDLER var.and(VAL_HANDLER_DEL).dqt()),
                    fmt!(SET_ADD GLOB VAL_HANDLER NIL.and(VAL_HANDLER_DEL).qt()),
                )
                .and(CMD_DEL)
            })
            .collect::<String>(),
    ]
    .join(CMD_DEL)
}

pub fn extract_val_handler(values: String) -> Vec<String> {
    values
        .split_terminator(VAL_HANDLER_DEL)
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
}

pub fn throw_error<A: Cmd, B: Cmd, C: Cmd>(client: A, fail_msg: B, error: C) -> String {
    cmd(
        fmt!(EVAL "-client" client),
        [
            fmt!(ECHO "-markup" "{Error}".and(fail_msg).kakqt()),
            fmt!(ECHO_DBG SELF.and("::ERR ").and(error.dqt()).kakqt()),
        ]
        .join(CMD_DEL),
    )
}

pub fn writeln_kakbuf<S: Cmd>(buffer: S, msg: S) -> String {
    let buffer = buffer.qt();
    [
        try_catch(fmt!["edit -existing" buffer], fmt!["edit -scratch" buffer]),
        fmt![SET_REG "g".qt() msg.dqt()],
        fmt![EXEC "gegh\"gP<a-o>".qt()],
    ]
    .join(CMD_DEL)
}

pub fn cmd<S: Cmd>(cmd_args: S, block: S) -> String {
    cmd_args.and(block.kakqt())
}

pub fn try_catch<S: Cmd>(try_cmd: S, catch_cmd: S) -> String {
    fmt!(TRY_CATCH[0] try_cmd.kakqt() TRY_CATCH[1] catch_cmd.kakqt())
}

pub trait Cmd: AsRef<str> {
    fn sur_with(&self, op: &str, cl: &str) -> String {
        let mut val = String::new();
        val.push_str(op);
        val.push_str(self.as_ref());
        val.push_str(cl);

        val
    }

    fn and<S: Cmd>(&self, more: S) -> String {
        let mut new = String::new();
        new.push_str(self.as_ref());
        new.push_str(more.as_ref());

        new
    }

    fn as_arg(&self) -> String {
        self.sur_with("%arg¿", "¿")
    }

    fn as_reg(&self) -> String {
        self.sur_with("%reg£", "£")
    }

    fn as_val(&self) -> String {
        self.sur_with("%val®", "®")
    }

    fn as_opt(&self) -> String {
        self.sur_with("%opt¶", "¶")
    }

    fn qt(&self) -> String {
        self.sur_with("'", "'")
    }

    fn dqt(&self) -> String {
        self.sur_with("\"", "\"")
    }

    fn kakqt(&self) -> String {
        self.sur_with("%[", "]")
    }

    fn catch_err<S: Cmd>(&self, err_cmd: S) -> String {
        try_catch(self.as_ref(), err_cmd.as_ref())
    }
}

impl Cmd for String {}
impl Cmd for &String {}
impl Cmd for &str {}

// fn main() {
//     // println!("{}", throw_error("client0", "this is err msg"));
// 	// println!("{}", session_prelude());
// 	println!("{}", throw_error("client", "errmsg", "context"));
// }
