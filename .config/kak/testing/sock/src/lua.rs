use crate::kak_cmd;
use crate::kak_cmd::SELF;
use crate::utils::send_to_kak_socket;
use crate::GluaClient;
pub use mlua::Lua;
use mlua::{ChunkMode, FromLua, Function, Result, Table, UserData, Value, Variadic};

const OPS_TABLE: &str = "operations_stack";

#[derive(Clone)]
pub enum Operation {
    RawEval(String),
    ToSocket(String, String),
    Nop,
}

impl UserData for Operation {}

pub trait LuaState {
    fn init_state() -> Result<Lua>;
    fn eval_and_get_ops(&self, chunk: String) -> Result<Vec<Operation>>;
}

impl LuaState for Lua {
    fn init_state() -> Result<Lua> {
        let state = Lua::new();
        let globals = state.globals();

        let user_stuff = state.create_table()?;
        let ops_table = state.create_table()?;
        let funcs = state.create_table()?;

        funcs.set(
            "raw_eval",
            state.create_function(|lua, cmd: String| {
                lua.globals()
                    .get::<_, Table>(OPS_TABLE)?
                    .push(Operation::RawEval(cmd))?;
                Ok(())
            })?,
        )?;

        funcs.set(
            "send_to",
            state.create_function(|lua, (ses, msg): (String, String)| {
                lua.globals()
                    .get::<_, Table>(OPS_TABLE)?
                    .push(Operation::ToSocket(ses, msg))?;
                Ok(())
            })?,
        )?;

        funcs.set(OPS_TABLE, ops_table)?;
        funcs.set("user", user_stuff)?;

        globals.set("kak", funcs)?;
        drop(globals);

        Ok(state)
    }

    fn eval_and_get_ops(&self, chunk: String) -> Result<Vec<Operation>> {
        self.load(&chunk).into_function()?;
        // TODO: Load chunck as functions that returns table with Operations
        // 		Table will have at least one item: Nop
        // 		Better idea?
        let ops = self.globals().get::<_, Table>(OPS_TABLE)?;

        Ok(ops.sequence_values().map(|op| op.unwrap()).collect::<Vec<Operation>>())
    }
}

// glua-eval 'kak.raw_eval("hello world")'
// glua-eval 'kak.send_to("sock", "msg")'
