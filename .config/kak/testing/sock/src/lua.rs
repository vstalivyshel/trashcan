use crate::utils::send_to_kak_socket;
use mlua::{ChunkMode, Function, Lua, Result, Value, Variadic};

pub fn lua_exec(chunck: String) -> Result<()> {
    let lua = Lua::new();
    let globals = lua.globals();

    let functions = lua.create_table()?;

    let send_socket = lua.create_function(|_, (session, msg): (String, String)| {
        let _ = send_to_kak_socket(&session, &msg);

        Ok(())
    })?;

    functions.set("send_socket", send_socket)?;
    globals.set("kak", functions)?;

    lua.load(&chunck)
        .set_name("Glua")?
        .set_mode(ChunkMode::Text)
        .exec()?;

    Ok(())
}
