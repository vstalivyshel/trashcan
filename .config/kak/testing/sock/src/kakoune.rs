use crate::{f, utils::*, SELF, VAL_HANLDR};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::unix::net::UnixStream,
    path::Path,
};
// TODO: deal with temp fifo

const VAL_SEP: &str = "§";

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
    let fifo = temp_fifo_in(Path::new(temp_path))?;

    let mut cmd =
        f!("declare-option -hidden str" VAL_HANLDR "; set-option global" VAL_HANLDR "''; ");

    for var in vars.into_iter() {
        let var = var.to_string();
        cmd.push_str(&try_catch(
            f!("set -add global" VAL_HANLDR var.and(VAL_SEP).dqt()),
            f!("set -add global" VAL_HANLDR "nil".and(VAL_SEP).dqt()),
        ));
        cmd.push_str("; ");
    }

    let fifo_path = fifo.path.to_str().unwrap();
    cmd.push_str(&f!("echo -to-file" fifo_path.qt() VAL_HANLDR.as_opt().dqt()));

    kak_send_client(session, client, &cmd)?;

    let mut values = String::new();
    File::open(&fifo.path)?.read_to_string(&mut values)?;

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
