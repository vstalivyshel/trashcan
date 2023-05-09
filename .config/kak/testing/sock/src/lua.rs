use crate::{SELF, VAL_SEP};
use crate::f;
use crate::kak_cmd::{evacl, try_catch, Cmd, Prefix};
use crate::utils::{create_fifo, send_to_kak_socket};
pub use mlua::Lua;
use mlua::{FromLua, Result, Table, ToLua, Variadic};
use std::{fs::File, io::Read, path::Path};

pub const KAK: &str = "kak";
pub const SES: &str = "session";
pub const CLIENT: &str = "client";
pub const META: &str = "current";
pub const ROOT: &str = "root_dir";


pub trait LuaServer {
    fn server_prelude(&self) -> Result<()>;
    fn session_data(&self) -> Result<Table>;
    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()>;
    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A>;
    fn kak_get(&self, prefix: Prefix, var_name: Variadic<String>) -> Result<Table>;
    fn kak_eval<A: Cmd>(&self, cmd: A) -> Result<()>;
    fn exec(&self, chunck: String) -> Result<()>;
}

impl LuaServer for Lua {
    fn server_prelude(&self) -> Result<()> {
        let globals = self.globals();
        let kak = self.create_table()?;
        let meta = self.create_table()?;
        kak.set(META, meta)?;

        kak.set(
            "eval",
            self.create_function(|lua, cmd: String| lua.kak_eval(cmd))?,
        )?;

        kak.set(
            "send_to",
            self.create_function(|_, (ses, msg): (String, String)| {
                send_to_kak_socket(&ses, &msg)?;
                Ok(())
            })?,
        )?;

        globals.set(KAK, kak)?;

        Ok(())
    }

    fn session_data(&self) -> Result<Table> {
        self.globals().get::<_, Table>(KAK)?.get::<_, Table>(META)
    }

    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()> {
        self.session_data()?.set(field, value)
    }

    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A> {
        self.session_data()?.get::<_, A>(field)
    }


    fn kak_eval<A: Cmd>(&self, cmd: A) -> Result<()> {
        let cur_client = self.get_data::<String>(CLIENT)?;
        let cur_ses = self.get_data::<String>(SES)?;
        send_to_kak_socket(&cur_ses, &evacl(cur_client, cmd))?;

        Ok(())
    }

    fn exec(&self, chunck: String) -> Result<()> {
        self.load(&chunck).exec()
    }
}
