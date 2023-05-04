#[macro_use]
mod lua;
mod kak_cmd;
mod utils;
use crate::kak_cmd::{Cmd, EVACL, EXEC, SELF};
use crate::lua::{lua_prelude, LuaServer};
use crate::utils::*;
use log::{debug, error, info};
use std::io::{BufRead, BufReader};
use std::process::{self, Stdio};

fn run() -> Result<(), mlua::Error> {
    let session = std::env::args().skip(1).next();
    if session.is_none() {
        eprintln!("No session name supplied");
        process::exit(69)
    }
    let session = session.unwrap();

    let mut json_client = std::process::Command::new("kak")
        .args([
            "-c",
            &session,
            "-ui",
            "json",
            "-e",
            &f!("rename-client" SELF "; e -scratch *scratch*"),
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stream = BufReader::new(json_client.stdout.take().unwrap());
    let mut output_buffer = Vec::<u8>::new();

    let lua = lua_prelude(&session).unwrap();

    loop {
        stream.read_until(b'\n', &mut output_buffer)?;
        let received = serde_json::from_slice::<JsonRpc>(&output_buffer);
        output_buffer.clear();

        if let Err(parse_err) = received {
            if parse_err.is_eof() {
                break;
            }
            continue;
        }

        let info = received.unwrap().params;
        lua.send_current_session(&f!(EVACL SELF).and_kakqt(f!(EXEC "<esc>")))?;

        let chunck = info.content();
        if chunck.is_empty() {
            continue;
        }

        let client = info.title_content();
        lua.set_client(&client)?;

        info!("InfoShow.title.content: \"{client}\"");
        info!("InfoShow.content.content: \"{chunck}\"");
        debug!("InfoShow: \n{info:?}");

        if let Err(lua_err) = lua.chunck_eval(&chunck) {
            error!("Lua::Error: \n{lua_err}");
            lua.send_current_session(&kak_cmd::throw_error(
                &client,
                "Error executing lua chunck! See debug",
                lua_err.to_string(),
            ))?;
        }
    }

    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(lua_err) = run() {
        eprintln!("{SELF}::LuaError: {lua_err}");
        std::process::exit(69);
    };
}
