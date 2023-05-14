use crate::kakoune::*;
use crate::ClientData;
pub use mlua::Lua;
use mlua::{FromLua, MultiValue, Result, Table, ToLua, Variadic};

const KAK: &str = "kak";
const SES: &str = "session";
const CLIENT: &str = "client";
const ROOT: &str = "root_dir";

pub trait LuaServer {
    fn prelude(&self, root: String) -> Result<()>;
    fn session_data(&self) -> Result<Table>;
    fn set_data<A: for<'a> ToLua<'a>>(&self, field: &str, value: A) -> Result<()>;
    fn get_data<A: for<'a> FromLua<'a>>(&self, field: &str) -> Result<A>;
    fn call_chunk(&self, data: ClientData) -> Result<Vec<String>>;
    fn mulit_value_from<I: IntoIterator<Item = String>>(&self, items: I) -> Result<MultiValue>;
    fn kak_eval(&self, cmd: String) -> Result<()>;
    fn kak_get(&self, vars: Variadic<String>) -> Result<MultiValue>;
}

impl LuaServer for Lua {
    fn prelude(&self, root: String) -> Result<()> {
        let globals = self.globals();
        let kak = self.create_table()?;
        kak.set(ROOT, root)?;

        kak.set(
            "send_to",
            self.create_function(|_, (ses, cmd): (String, String)| {
                kak_send_msg(&ses, &cmd)?;
                Ok(())
            })?,
        )?;

        kak.set(
            "eval",
            self.create_function(|lua, cmd: String| lua.kak_eval(cmd))?,
        )?;

        kak.set(
            "get",
            self.create_function(|lua, vars: Variadic<String>| lua.kak_get(vars))?,
        )?;

        globals.set(KAK, kak)?;

        Ok(())
    }

    fn mulit_value_from<I: IntoIterator<Item = String>>(&self, items: I) -> Result<MultiValue> {
        let mut result = MultiValue::new();
        for val in items.into_iter() {
            result.push_front(if let Ok(f) = val.parse::<f64>() {
                f.to_lua(&self)?
            } else if let Ok(i) = val.parse::<i64>() {
                i.to_lua(&self)?
            } else if let Ok(b) = val.parse::<bool>() {
                b.to_lua(&self)?
            } else {
                val.to_lua(&self)?
            });
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

    fn call_chunk(&self, data: ClientData) -> Result<Vec<String>> {
        let args = self.mulit_value_from(data.chunk_args)?;
        self.set_data::<String>(SES, data.session)?;
        self.set_data::<String>(CLIENT, data.client)?;
        let vals = self
            .load(&data.chunk)
            .call::<MultiValue, MultiValue>(args)?;
        let mut result = Vec::<String>::new();
        for val in vals.into_iter() {
            if let Ok(v) = String::from_lua(val, self) {
                result.push(v);
            } else {
                result.push("Unconverable".to_string());
            }
        }

        Ok(result)
    }

    fn kak_eval(&self, cmd: String) -> Result<()> {
        let cur_client = self.get_data::<String>(CLIENT)?;
        let cur_ses = self.get_data::<String>(SES)?;
        kak_send_client(&cur_ses, &cur_client, &cmd)?;

        Ok(())
    }

    fn kak_get(&self, vars: Variadic<String>) -> Result<MultiValue> {
        let root = self.get_data::<String>(ROOT)?;
        let cur_ses = self.get_data::<String>(SES)?;
        let cur_client = self.get_data::<String>(CLIENT)?;
        let vals = kak_get_values(&root, &cur_ses, &cur_client, vars)?;

        self.mulit_value_from(vals)
    }
}
