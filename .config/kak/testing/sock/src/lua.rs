use crate::send_to_kak_socket;
pub use mlua::Lua;
use mlua::{Result, Table};

pub const KAK: &str = "kak";
pub const META: &str = "META";

pub enum Prefix {
    Opt,
    Val,
    Arg,
    Reg,
}

trait LuaServer {
    fn current_session(&self) -> Result<String>;
    fn current_client(&self) -> Result<String>;
    fn set_client(&self, client: String) -> Result<()>;
    fn session_data(&self) -> Result<Table>;
}

impl LuaServer for Lua {
    fn current_session(&self) -> Result<String> {
        self.session_data()?.get::<_, String>("session")
    }

    fn current_client(&self) -> Result<String> {
        self.session_data()?.get::<_, String>("client")
    }

    fn set_client(&self, client: String) -> Result<()> {
        self.session_data()?.set("client", client)
    }

    fn session_data(&self) -> Result<Table> {
        self.globals().get::<_, Table>(KAK)?.get::<_, Table>(META)
    }
}

pub fn lua_prelude(session: &str) -> Result<Lua> {
    let lua = Lua::new();
    {
        let globals = lua.globals();
        let kak = lua.create_table()?;
        let meta = lua.create_table()?;
        meta.set("session", session.to_string())?;

        kak.set(META, session.to_string())?;

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

        // kak.set(
        //     "eval",
        //     lua.create_function(|lua, cmd: String| {
        //     })?,
        // )?;

        kak.set(
            "send",
            lua.create_function(|lua, msg: String| {
                send_to_kak_socket(&lua.current_session()?, &msg)?;
                Ok(())
            })?,
        )?;

        // kak.set(
        //     "send_to",
        //     lua.create_function(|lua, (ses, msg): (String, String)| {
        //         send_to_kak_socket(&ses, &msg)?;
        //         Ok(())
        //     })?,
        // )?;

        globals.set("kak", kak)?;
    }

    Ok(lua)
}
