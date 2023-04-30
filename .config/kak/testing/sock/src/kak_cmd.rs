use std::fmt::Display;

const SELF: &str = "GLUA";
const COMMAND: &str = "glua-eval";
const VAL_HANDLER: &str = "glua_val_handler";
const BLOCK_Q: [&str; 2] = ["%[", "]"];

pub struct ClientCmd {
    pub name: String,
}

impl ClientCmd {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn exec<S: Display>(&self, keys: S) -> String {
        let exec_cmd = format!("exec {keys}", keys = qt(keys));
        self.eval(&exec_cmd, err("Exec-Keys Failed", &exec_cmd))
    }

    pub fn echo<S: Display>(&self, msg: S) -> String {
        let echo_cmd = format!("echo {msg}");
        self.eval(&echo_cmd, err("Echo Failed", &echo_cmd))
    }

    pub fn info<D: Display, S: Display>(&self, title: D, msg: S) -> String {
        let info_cmd = format!("info -title {title} {msg}", msg = dqt(msg));
        self.eval(&info_cmd, err("InfoShow Failed", &info_cmd))
    }

    pub fn eval<D: Display, S: Display>(&self, cmd: D, err_cmd: S) -> String {
        format!(
            "eval -client {client} {cmd}",
            client = self.name,
            cmd = kakqt(try_catch(cmd, err_cmd))
        )
    }
}

fn writeln_kakbuf(buffer: &str, msg: &str) -> String {
    let open_buf = try_eval(
        try_catch(
            format!("e -existing {buffer}"),
            format!("e -scratch {buffer}"),
        ),
        "Failed to edit a buffer for command output",
    );

    let set_reg = try_eval(
        format!("set-register 'g' {msg}", msg = dqt(msg)),
        "Failed to set content to a register while writing to a buffer",
    );

    let exec = try_eval(
        "exec 'gegh\"gP<a-o>'",
        "Failed to paste content into a buffer",
    );

    format!("eval -save-regs 'g' -client GLUA %[ {open_buf}; {set_reg}; {exec}; ]")
}

pub fn server_prelude() -> String {
    let decl_handler = try_eval(
        format!("declare-option -hidden str {VAL_HANDLER}"),
        "Failed to decalre main value handler for server!",
    );

    let def_main = try_def(
        format!("{COMMAND} -override -params .."),
        format!(
            "eval -client {SELF} %[ info -title {client} {content} ]",
            client = val("client"),
            content = arg("@")
        ),
        "Failed to define main server command!",
    );

    format!("{decl_handler}; {def_main}")
}

pub fn set_val_handler<S: Display, I: IntoIterator<Item = S>>(vars: I) -> String {
    let set_add = format!("set -add global {VAL_HANDLER}");
    let set_nil = format!("{set_add} 'nil'");
    let mut cmd = String::new();
    for var in vars.into_iter() {
        let set_cmd = format!("{set_add} {var}");
        cmd.push_str(&try_catch(
            &set_cmd,
            try_catch(&set_nil, err("Set-Option Failed", &set_cmd)),
        ));
        cmd.push_str(&format!(" {set_add} '§'; "));
    }

    format!(" set gloabl {VAL_HANDLER} ''; {cmd}")
}

pub fn try_def<D: Display, S: Display, E: Display>(command: D, block: S, err_msg: E) -> String {
    try_eval(
        format!("define-command {command} {block}", block = kakqt(block)),
        err_msg,
    )
}

pub fn try_eval<D: Display, S: Display>(cmd: D, err_msg: S) -> String {
    try_catch(&cmd, err(err_msg, &cmd))
}

pub fn try_catch<D: Display, S: Display>(try_cmd: D, catch_cmd: S) -> String {
    format!(
        "try {try_cmd} catch {catch_cmd}",
        try_cmd = kakqt(try_cmd),
        catch_cmd = kakqt(catch_cmd)
    )
}

pub fn err<D: Display, S: ToString>(err_msg: D, ctx: S) -> String {
    let ctx_pretty = debug_kakcmd(ctx.to_string());
    format!(
        "\
echo -debug {SELF}::KakErr: {kak_err}; \
echo -debug {SELF}::Debug: Err somewhere here >>; \
{ctx_pretty}\
fail {SELF}::Err: {err_msg}, see debug;",
        err_msg = qt(err_msg),
        kak_err = dqt("%val{error}"),
    )
}

pub fn with_dqt<S: Display>(val: S) -> String {
    format!("\\\'{val}\\\'")
}

pub fn arg<S: Display>(val: S) -> String {
    format!("%arg¿{val}¿")
}

pub fn reg<S: Display>(val: S) -> String {
    format!("%reg£{val}£")
}

pub fn val<S: Display>(val: S) -> String {
    format!("%val®{val}®")
}

pub fn opt<S: Display>(val: S) -> String {
    format!("%opt¶{val}¶")
}

pub fn qt<S: Display>(val: S) -> String {
    format!("'{val}'")
}

pub fn dqt<S: Display>(val: S) -> String {
    format!("\"{val}\"")
}

pub fn kakqt<S: Display>(val: S) -> String {
    format!("%[ {val} ]")
}

fn debug_kakcmd<S: ToString>(ctx: S) -> String {
    fn push_line(depth: usize, line: &mut String, lines: &mut Vec<String>) {
        lines.push(format!("echo -debug %{{{line}}};"));
        line.clear();
        let indent_newline = "  ";
        if depth > 0 {
            for _ in 0..=depth {
                line.push_str(indent_newline);
            }
        }
    }

    let mut ctx = ctx.to_string();
    let mut depth = 0 as usize;
    let mut words = ctx.split_whitespace();

    let block_open = BLOCK_Q[0];
    let block_close = BLOCK_Q[1];

    let mut line = String::new();
    let mut lines = Vec::<String>::new();

    for (idx, word) in words.clone().enumerate() {
        if word.contains(block_open) {
            depth += 1;
            line.push_str(word);
            push_line(depth, &mut line, &mut lines);
        } else if word.contains(block_close) {
            // This doesn't work
            // if let Some(keyword) = words.nth(idx + 1) {
            //     if keyword.contains("catch") {
            //         line.push_str(word);
            //         line.push(' ');
            //         continue;
            //     }
            // }
            depth -= 1;
            push_line(depth, &mut line, &mut lines);
            line.push_str(word);
            push_line(depth, &mut line, &mut lines);
        } else if word.contains(';') {
            line.push_str(word);
            push_line(depth, &mut line, &mut lines);
        } else {
            line.push_str(word);
            line.push(' ');
        }
    }
	lines.join("\n")
}

fn main() {
    let glua = ClientCmd::new("GLUA");
    let cmd = set_val_handler([opt("suka"), val("blyad"), dqt("some text in dqt")]);
    let prelude = server_prelude();
    let write_buf = writeln_kakbuf("some_buffer", "this is msg\nfor some buffer\ntyes, no, uska blyad");
    // println!("{}", try_eval("echo %val{session}", "FAiled to session"))
    // debug_cmd(&cmd);
    // println!("{write_buf}");
    println!("{}", debug_kakcmd(&write_buf));
}

