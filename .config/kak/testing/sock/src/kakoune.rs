use crate::{f, utils::*, SELF, SOCK_HANDLER};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::unix::net::UnixStream,
};

const VAL_SEP: &str = "§";

pub fn kak_init_cmd(self_cmd: &str, root: &str) -> String {
    let cmd = "glua-eval";
    let cmd_sync = "glua-eval-sync";
    let cmd_kill = "glua-kill";
    let sock_handl_sh = &"$kak_opt_".and(SOCK_HANDLER).dqt();
    "provide-module -override glua-server".block(
    [
        f!("declare-option str" SOCK_HANDLER root.qt()),
        f!("define-command" cmd "-override -params 1..").and_kakqt(
            "evaluate-commands".and_sh([
                self_cmd,
                "send",
                sock_handl_sh,
                &"$kak_session".dqt(),
                &"$kak_client".dqt(),
                &"$@".dqt(),
            ]),
        ),
        f!("define-command" cmd_sync "-override -params 1..").and_kakqt(
            "evaluate-commands".and_sh([
                self_cmd,
                "sendsync",
                sock_handl_sh,
                &"$kak_session".dqt(),
                &"$kak_client".dqt(),
                &"$@".dqt(),
            ]),
        ),
        f!("define-command" cmd_kill "-override").block([
            "evaluate-commands".and_sh([
                self_cmd,
                "kill",
                sock_handl_sh,
            ]),
            f!("set-option global" SOCK_HANDLER "''"),
        ]),
    ])
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

pub fn kak_get_values<I: IntoIterator<Item = String>>(
    temp_path: &str,
    session: &str,
    client: &str,
    vars: I,
) -> Result<Vec<String>, io::Error> {
    let fifo: tempfile::TempPath;
    loop {
        match temp_fifo_in(std::path::Path::new(temp_path)) {
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

    for var in vars.into_iter() {
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

pub fn kak_throw_error<A: StringExt, B: StringExt>(
    session: &str,
    client: &str,
    fail_msg: A,
    error: B,
) -> Result<(), io::Error> {
    kak_send_client(
        session,
        client,
        &[
            "echo -markup".and_kakqt("{Error}".and(fail_msg).and(", see debug!")),
            "echo -debug".and_kakqt(SELF.and("::Error: ").and(error)),
        ]
        .as_cmd(),
    )
}
