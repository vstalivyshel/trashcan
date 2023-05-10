use crate::utils::*;
use crate::{f, MAIN_CMD, SELF};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::unix::net::UnixStream,
};

const VAL_SEP: &str = "§";

pub enum KakVal {
    Val(String),
    Opt(String),
    Reg(String),
    File(String),
}

impl ToString for KakVal {
    fn to_string(&self) -> String {
        use KakVal::*;
        match self {
            Val(v) => v.as_val(),
            Opt(v) => v.as_opt(),
            Reg(v) => v.as_reg(),
            File(v) => v.as_file(),
        }
    }
}

pub fn kak_init_cmd(self_cmd: &str, socket_path: &str) -> String {
    let socket_handler = &SELF.and("_socket_handler");
    let sh_socket_handler = &"$kak_opt_".and(socket_handler);
    let glua = "glua-eval";
    [
        f!("declare-option str" socket_handler),
        f!("set-option global" socket_handler socket_path.qt()),
        f!("define-command" glua "-override -params 1..").and_kakqt("evaluate-commands".and_sh([
            self_cmd.to_string(),
            sh_socket_handler.clone(),
            "$kak_session".dqt(),
            "$kak_client".dqt(),
            "$@".dqt(),
        ])),
        f!("define-command glua-kill-server -override").and_kakqt("evaluate-commands".and_sh([
            self_cmd,
            "kill",
            sh_socket_handler,
        ])),
        f!("alias global lua" glua),
        f!("hook global -group" SELF.and("-kill-yourself").qt() "KakEnd '.*'").and_sh([
            self_cmd,
            "kill",
            sh_socket_handler,
        ]),
    ]
    .as_cmd()
}

pub fn kak_send_msg(session: &str, msg: &str) -> Result<(), io::Error> {
    let rntm = std::env::var("XDG_RUNTIME_DIR").expect("runtimedir");
    let socket = std::path::PathBuf::from(rntm).join("kakoune").join(session);
    let mut stream = UnixStream::connect(socket)?;
    stream.write(&encode(msg))?;
    stream.flush()?;

    Ok(())
}

pub fn kak_send_client(session: &str, client: &str, msg: &str) -> Result<(), io::Error> {
    kak_send_msg(session, &f!("evaluate-commands -client" client msg.kakqt()))
}

pub fn kak_get_values(
    session: &str,
    client: &str,
    vars: Vec<KakVal>,
) -> Result<Vec<String>, io::Error> {
    let fifo: tempfile::TempPath;
    loop {
        match temp_fifo() {
            Some(path) => {
                fifo = path;
                break;
            }
            None => continue,
        }
    }

    let val_handle = SELF.and("_value_handler");
    let mut cmd =
        f!("declare-option -hidden str" val_handle "; set-option global" val_handle "''; ");

    for var in vars.as_slice() {
        let var = var.to_string();
        cmd.push_str(&try_catch(
            f!("set -add global" val_handle var.and(VAL_SEP).dqt()),
            f!("set -add global" val_handle "nil".and(VAL_SEP).dqt()),
        ));
        cmd.push_str("; ");
    }

    let fifo_path = fifo.to_str().unwrap();
    cmd.push_str(&f!("echo -to-file" fifo_path.qt() val_handle.as_opt().dqt()));

    kak_send_client(session, client, &cmd)?;

    let mut values = String::new();
    File::open(fifo)?.read_to_string(&mut values)?;

    Ok(values
        .split_terminator(VAL_SEP)
        .map(|v| v.to_string())
        .collect::<Vec<String>>())
}

pub fn kak_throw_error<B: StringExt, C: StringExt>(
    session: &str,
    client: &str,
    fail_msg: B,
    error: C,
) -> Result<(), io::Error> {
    kak_send_client(
        session,
        client,
        &[
            "echo -markup".and_kakqt("{Error}".and(fail_msg).and(", see debug!")),
            "echo -debug".and_kakqt(SELF.and("::ERR\n").and(error)),
        ]
        .as_cmd(),
    )
}
