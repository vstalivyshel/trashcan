use crate::f;
use crate::utils::send_to_kak_socket;
use crate::kak_cmd::{self, Cmd, EVACL, Prefix};
pub use mlua::Lua;
use mlua::{Result, Table, Variadic};

pub const KAK: &str = "kak";
pub const META: &str = "current";
pub const RECV: &str = "received";
const SES: &str = "session";
const CLIENT: &str = "client";

pub trait LuaServer {
    fn current_session(&self) -> Result<String>;
    fn current_client(&self) -> Result<String>;
    fn received_values(&self) -> Result<Table>;
    fn set_client(&self, client: &str) -> Result<()>;
    fn session_data(&self) -> Result<Table>;
    fn chunck_eval(&self, chunck: &str) -> Result<()>;
    fn kak_eval_current_session(&self, cmd: &str) -> Result<()>;
    fn kak_eval_client(&self, client: &str, cmd: &str) -> Result<()>;
    fn kak_eval_current_client(&self, cmd: &str) -> Result<()>;
}

impl LuaServer for Lua {
    fn session_data(&self) -> Result<Table> {
        self.globals().get::<_, Table>(KAK)?.get::<_, Table>(META)
    }

    fn current_session(&self) -> Result<String> {
        self.session_data()?.get::<_, String>(SES)
    }

    fn current_client(&self) -> Result<String> {
        self.session_data()?.get::<_, String>(CLIENT)
    }

    fn received_values(&self) -> Result<Table> {
        self.session_data()?.get::<_, Table>(RECV)
    }

    fn set_client(&self, client: &str) -> Result<()> {
        self.session_data()?.set(CLIENT, client.to_string())
    }

    fn chunck_eval(&self, chunck: &str) -> Result<()> {
        self.load(&chunck.to_string()).eval()?;
        let empty = self.create_table()?;
        self.session_data()?.set(RECV, empty)
    }

    fn kak_eval_current_session(&self, msg: &str) -> Result<()> {
        send_to_kak_socket(&self.current_session()?, msg)?;
        Ok(())
    }

    fn kak_eval_client(&self, client: &str, cmd: &str) -> Result<()> {
        self.kak_eval_current_session(&f!(EVACL client cmd.kakqt()))
    }

    fn kak_eval_current_client(&self, cmd: &str) -> Result<()> {
        self.kak_eval_client(&self.current_client()?, &cmd.kakqt())
    }
}

pub fn lua_prelude(session: &str) -> Result<Lua> {
    let lua = Lua::new();
    {
        let globals = lua.globals();
        let kak = lua.create_table()?;
        let meta = lua.create_table()?;
        let received_values = lua.create_table()?;
        meta.set(RECV, received_values)?;
        meta.set(SES, session.to_string())?;

        kak.set(META, meta)?;

        kak.set(
            "val",
            lua.create_function(|lua, vars: Variadic<String>| {
                lua.kak_eval_current_client(&kak_cmd::request_value(Prefix::Val, vars.as_slice()))?;
                Ok(lua.received_values()?)
            })?,
        )?;

        kak.set(
            "eval",
            lua.create_function(|lua, cmd: String| lua.kak_eval_current_client(&cmd))?,
        )?;

        kak.set(
            "send",
            lua.create_function(|lua, msg: String| lua.kak_eval_current_session(&msg))?,
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
