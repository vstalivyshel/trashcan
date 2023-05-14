#[test]
fn get_vals() {
    use crate::kak_get_values;
    let v = kak_get_values("./", "sock", "client0", ["%val{client}".into(), "%val{session}".into()]);
    assert!(v.is_ok());
    assert!(v.unwrap().len() == 2);
}

#[test]
fn right_data_lua() {
    use crate::lua::*;
    use crate::{TEMP_FIFO, TempFifo};
    use crate::ClientData;
    use crate::utils::*;
    let lua = Lua::new();
    let root = "./suka.glua.root/".to_string();
    lua.prelude(root.clone()).unwrap();
    let chunk = r#"
        local client = kak.client
        local server = kak.server
        local root = kak.root_dir
        return root
    "#;
    let data = ClientData {
        session: "sock".into(),
        client: "client0".into(),
        chunk: chunk.into(),
        chunk_args: Vec::new(),
    };
    let path = std::path::Path::new(&root)
        .join(TEMP_FIFO)
        .with_extension(format!("{:?}", rand::random::<u64>()));

	let fifo = TempFifo { path: path.clone() };
    let v = lua.call_chunk(data).unwrap();
    assert!(create_fifo(&path, 0o777).is_ok());
	assert_eq!(v.get(0).unwrap(), &root);
}

#[test]
fn get_vals_lua() {
    use crate::lua::*;
    use crate::ClientData;
    let lua = Lua::new();
    lua.prelude("./suka.glua.root/".into()).unwrap();
    let chunk = r#"
        local vals = kak.get("%val{session}")
        return vals
    "#;
    let data = ClientData {
        session: "sock".into(),
        client: "client0".into(),
        chunk: chunk.into(),
        chunk_args: Vec::new(),
    };

    let v = lua.call_chunk(data).unwrap();

    assert!(v.len() == 1);
    assert_eq!(v.get(0).unwrap(), "sock");
}
