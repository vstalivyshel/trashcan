use crate::f;
use crate::send_to_kak_socket;
use crate::{Cmd, EVACL};
pub use mlua::Lua;
use mlua::{Result, Table};

pub const KAK: &str = "kak";
pub const META: &str = "current";

pub enum Prefix {
    Opt,
    Val,
    Arg,
    Reg,
}

pub trait LuaServer {
    fn current_session(&self) -> Result<String>;
    fn current_client(&self) -> Result<String>;
    fn set_client(&self, client: &str) -> Result<()>;
    fn session_data(&self) -> Result<Table>;
    fn chunck_eval(&self, chunck: &str) -> Result<()>;
    fn send_current_session(&self, msg: &str) -> Result<()>;
    fn kak_eval_current_session(&self, cmd: &str) -> Result<()>;
}

impl LuaServer for Lua {
    fn session_data(&self) -> Result<Table> {
        self.globals().get::<_, Table>(KAK)?.get::<_, Table>(META)
    }

    fn current_session(&self) -> Result<String> {
        self.session_data()?.get::<_, String>("session")
    }

    fn current_client(&self) -> Result<String> {
        self.session_data()?.get::<_, String>("client")
    }

    fn set_client(&self, client: &str) -> Result<()> {
        self.session_data()?.set("client", client.to_string())
    }

    fn chunck_eval(&self, chunck: &str) -> Result<()> {
        self.load(&chunck.to_string()).eval()
    }

    fn send_current_session(&self, msg: &str) -> Result<()> {
        send_to_kak_socket(&self.current_session()?, msg)?;
        Ok(())
    }

    fn kak_eval_current_session(&self, cmd: &str) -> Result<()> {
        self.send_current_session(&f!(EVACL self.current_client()? cmd.kakqt()))
    }
}

pub fn lua_prelude(session: &str) -> Result<Lua> {
    let lua = Lua::new();
    {
        let globals = lua.globals();
        let kak = lua.create_table()?;
        let meta = lua.create_table()?;
        meta.set("session", session.to_string())?;

        kak.set(META, meta)?;

        // kak.set(
        //     "val",
        //     lua.create_function(|lua, vals: Variadic<String>| {
        //     })?,
        // )?;

        // kak.set(
        //     "opt",
        //     lua.create_function(|lua, vals: Variadic<String>| {
        //     })?,
        // )?;

        // kak.set(
        // "reg",
        // lua.create_function(|lua, vals: Variadic<String>| {
        // })?,
        // )?;

        // kak.set(
        //     "arg",
        //     lua.create_function(|lua, vals: Variadic<String>| {
        //     })?,
        // )?;

        kak.set(
            "eval",
            lua.create_function(|lua, cmd: String| lua.kak_eval_current_session(&cmd))?,
        )?;

        kak.set(
            "send",
            lua.create_function(|lua, msg: String| lua.send_current_session(&msg))?,
        )?;

        kak.set(
            "send_to",
            lua.create_function(|_, (ses, msg): (String, String)| {
                send_to_kak_socket(&ses, &msg)?;
                Ok(())
            })?,
        )?;

        globals.set("kak", kak)?;
    }

    Ok(lua)
}
