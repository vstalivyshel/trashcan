#[macro_use]
mod lua;
mod kak_cmd;
mod utils;
use crate::kak_cmd::{Cmd, EVACL, EXEC, SELF, VAL_SEP};
use crate::lua::{lua_prelude, LuaServer};
use crate::utils::*;
use log::{debug, error, info};
use std::io::{BufRead, BufReader};
use std::process::{self, Stdio, ChildStdout};

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
    let lua = lua_prelude(&session)?;

    loop {
        let received = read_response(&mut stream, &mut output_buffer);

        if let Err(parse_err) =  received {
    		if parse_err.is_eof() {
        		break;
    		}
    		continue;
        }

        let info = received.unwrap().params;
        lua.kak_eval_client(SELF, "execute-keys '<esc>'")?;

        let chunck = info.content();
        if chunck.is_empty() {
            continue;
        } 

        let client = info.title_content();

        if client.starts_with("VALS") {
            for val in chunck.split_terminator(VAL_SEP) {
                lua.received_values()?.push(val.to_string())?;
            }
        } else if !client.is_empty() {
            lua.set_client(&client)?;
        }

        info!("InfoShow.title.content: \"{client}\"");
        info!("InfoShow.content.content: \"{chunck}\"");
        debug!("InfoShow: \n{info:?}");

        if let Err(lua_err) = lua.chunck_eval(&chunck) {
            error!("Lua::Error: \n{lua_err}");
            lua.kak_eval_current_client(&kak_cmd::throw_error(
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
