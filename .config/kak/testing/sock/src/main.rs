#[macro_use]
mod utils;
mod kak_cmd;
mod lua;
use crate::{
    kak_cmd::{Cmd, EVACL},
    lua::{SES, lua_prelude, Lua, LuaServer},
    utils::*,
};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
    process::{self, Stdio},
    thread::sleep,
    time::Duration,
};
use tempfile::TempDir;

// TODO: Test a lua fifo communication

pub const SELF: &str = "GLUA";
pub const VAL_FIFO: &str = "glua_val_fifo_handler";

struct GluaServer {
    session: String,
    root_dir: TempDir,
    val_fifo: PathBuf,
    val_fifo_path: String,
    lua: Lua,
}

impl GluaServer {
    pub fn setup(session: String) -> Result<Self, mlua::Error> {
        let root_dir = tempfile::Builder::new().prefix(SELF).tempdir()?;
        let mode = 0o777;
        let root_path = root_dir.path();
        let val_fifo = root_path.join("glua_pipe_A");
        create_fifo(&val_fifo, mode)?;
        let val_fifo_path = val_fifo.to_str().unwrap().to_string();
        let lua = lua_prelude()?;
        lua.set_data::<String>(SES, session.clone())?;

        Ok(GluaServer {
            session,
            root_dir,
            val_fifo,
            val_fifo_path,
            lua,
        })
    }

    fn run(&self) -> Result<(), mlua::Error> {
  //       println!("{}", &self.val_fifo_path);
  //       let mut output_buf = String::new();
  //       self.ask_for_value("client0", "buffile")?;
  //       // sleep(Duration::from_secs(1));
  //       let mut val_fifo_buf = BufReader::new(File::open(&self.val_fifo)?);
  //       val_fifo_buf.read_to_string(&mut output_buf)?;

		// println!("{output_buf}");
  //       // if !output_buf.is_empty() {
  //       //     if output_buf.contains("stop") {
  //       //         break
  //       //     }
  //       // }

  //       output_buf.clear();

        Ok(())
    }
}

fn main() {
    let supplied_session = std::env::args().skip(1).next();
    let session = if let Some(session) = supplied_session {
        session
    } else {
        eprintln!("{SELF}::Error => No session name supplied");
        process::exit(69);
    };

    let server = match GluaServer::setup(session) {
        Err(setup_failed) => {
            eprintln!("{SELF}::Error => Setup failed: \"{setup_failed}\"");
            process::exit(69);
        }
        Ok(nice) => nice,
    };

    if let Err(run_err) = server.run() {
        send_to_kak_socket(
            &server.session,
            &kak_cmd::throw_error(
                SELF.and("::Error => Something failed while running server! See debug!"),
                run_err.to_string(),
            ),
        )
        .expect("send 'run_err' to kakoune session");
    }
}
