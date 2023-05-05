use crate::f;
use crate::kak_cmd::{Cmd, EVACL};
use crate::utils::send_to_kak_socket;
pub use mlua::Lua;
use mlua::{Result, Table, ToLua, FromLua};
use std::io::{Read};
use std::fs::File;

pub const KAK: &str = "kak";
pub const SES: &str = "session";
pub const CLIENT: &str = "client";
pub const META: &str = "current";
pub const FIFO: &str = "val_fifo";

pub trait LuaServer {
    fn session_data(&self) -> Result<Table>;
    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()>;
    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A>;
    fn exec(&self, chunck: &str, args: Table) -> Result<()>;
    fn kak_eval(&self, cmd: &str) -> Result<()>;
}

impl LuaServer for Lua {
    fn session_data(&self) -> Result<Table> {
        self.globals().get::<_, Table>(KAK)?.get::<_, Table>(META)
    }

    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()> {
        self.session_data()?.set(field, value)
    }

    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A> {
        self.session_data()?.get::<_, A>(field)
    }

    fn exec(&self, chunck: &str, args: Table) -> Result<()> {
        self.load(&chunck.to_string()).exec()
    }

    fn kak_eval(&self, cmd: &str) -> Result<()> {
        let cur_ses = self.get_data::<String>(SES)?;
        send_to_kak_socket(&cur_ses, cmd)?;
        Ok(())
    }
}

pub fn lua_prelude() -> Result<Lua> {
    let lua = Lua::new();
    {
        let globals = lua.globals();
        let kak = lua.create_table()?;
        let meta = lua.create_table()?;
        kak.set(META, meta)?;

		kak.set(
    		"val",
    		lua.create_function(|lua, var: String|{
        		let fifo_path = &lua.get_data::<String>(FIFO)?;
        		let cur_client = lua.get_data::<String>(CLIENT)?;
        		lua.kak_eval(&f!(EVACL cur_client f!("echo -to-file" fifo_path.qt() var.as_val()).kakqt()))?;
        		let mut fifo = File::open(fifo_path)?;
        		let mut value = String::new();
        		fifo.read_to_string(&mut value)?;

        		Ok(value)
    		})?
		)?;

        kak.set(
            "send_to",
            lua.create_function(|_, (ses, msg): (String, String)| {
                send_to_kak_socket(&ses, &msg)?;
                Ok(())
            })?,
        )?;

        globals.set(KAK, kak)?;
    }

    Ok(lua)
}
