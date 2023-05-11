use crate::kakoune::*;
use crate::ClientData;
pub use mlua::Lua;
use mlua::{FromLua, Result, Table, ToLua, Variadic};

const KAK: &str = "kak";
const SES: &str = "session";
const CLIENT: &str = "client";
const ROOT: &str = "root_dir";

pub trait LuaServer {
    fn prelude(&self, root: String) -> Result<()>;
    fn session_data(&self) -> Result<Table>;
    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()>;
    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A>;
    fn load_data(&self, data: ClientData) -> Result<()>;
    fn table_from<I: IntoIterator<Item = String>>(&self, items: I) -> Result<Table>;
    fn kak_eval(&self, cmd: String) -> Result<()>;
    fn kak_get(&self, vars: Variadic<String>) -> Result<Table>;
}

impl LuaServer for Lua {
    fn prelude(&self, root: String) -> Result<()> {
        let globals = self.globals();
        let kak = self.create_table()?;
        kak.set(ROOT, root)?;

        kak.set(
            "eval",
            self.create_function(|lua, cmd: String| lua.kak_eval(cmd))?,
        )?;

        kak.set(
            "send_to",
            self.create_function(|_, (ses, cmd): (String, String)| {
                kak_send_msg(&ses, &cmd)?;
                Ok(())
            })?,
        )?;

        globals.set(KAK, kak)?;

        Ok(())
    }

    fn table_from<I: IntoIterator<Item = String>>(&self, items: I) -> Result<Table> {
        let result = self.create_table()?;
        for val in items.into_iter() {
            result.push(if let Ok(f) = val.parse::<f64>() {
                f.to_lua(&self)?
            } else if let Ok(i) = val.parse::<i64>() {
                i.to_lua(&self)?
            } else if let Ok(b) = val.parse::<bool>() {
                b.to_lua(&self)?
            } else {
                val.to_lua(&self)?
            })?;
        }

        Ok(result)
    }

    fn session_data(&self) -> Result<Table> {
        self.globals().get::<_, Table>(KAK)
    }

    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()> {
        self.session_data()?.set(field, value)
    }

    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A> {
        self.session_data()?.get::<_, A>(field)
    }

    fn load_data(&self, data: ClientData) -> Result<()> {
        let args = self.table_from(data.chunck_args)?;
        self.set_data::<String>(SES, data.session)?;
        self.set_data::<String>(CLIENT, data.client)?;
        self.load(&data.chunck).call(args)
    }

    fn kak_eval(&self, cmd: String) -> Result<()> {
        let cur_client = self.get_data::<String>(CLIENT)?;
        let cur_ses = self.get_data::<String>(SES)?;
        kak_send_client(&cur_ses, &cur_client, &cmd)?;

        Ok(())
    }

    fn kak_get(&self, vars: Variadic<String>) -> Result<Table> {
        let root = self.get_data::<String>(ROOT)?;
        let cur_ses = self.get_data::<String>(SES)?;
        let cur_client = self.get_data::<String>(CLIENT)?;
        let vals = kak_get_values(&root, &cur_ses, &cur_client, vars)?;

        self.table_from(vals)
    }
}
