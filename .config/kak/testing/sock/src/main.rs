use libc::{mkfifo, c_char};
use std::ffi::CString;
use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::fs::{self, OpenOptions};

struct Glua {
	vars_fifo_path: String,
	main_fifo_path: String,
	kak_vars_handler: String,
	debug: bool,
}

impl Glua {
    fn new(glua_temp_dir: &str) -> Self {
        let main_fifo_path = glua_temp_dir.to_string().push_str("glua_main_fifo");
        let vars_fifo_path = glua_temp_dir.to_string().push_str("glua_");
        let kak_vars_handler = "kak_glua_vars".to_string();

    	create_fifo(main_fifo_path, 444);
    	create_fifo(var_fifo_path, 444);

    }

    fn get_var(&self, client: &str, session: &str var_name: &str) -> String {
        let var_fifo_path = self.vars_fifo_path;
		let vars_handler = self.kak_vars_handler;

    	let mut cmd = format!("eval -client {client} %[
        declare-option str-list {vars_handler} ;
        set global {vars_handler}
    	try %[ set -add global {vars_handler} {var} ] catch %[ set -add global {vars_handler} nil ] ;
        echo -to-file {var_fifo_path} %opt[{vars_handler}] ;
    	]");

    	if self.debug {
        	println!("DEBUG::get_var()\n{cmd}");
    	}

    	talk_to_kak(session, cmd);

    	let var_fifo = OpenOptions::new()
        	.read(true)
        	.open(var_fifo_path)
        	.unwrap();
    	let mut var_reader = BufReader::new(var_fifo);
    	let mut var_val = cmd.clear();
    	var_reader.read_to_string(&mut var_val).unwrap();

    	var_val
    }

    fn get_vars_vec(&self, client: &str, session: &str, var_names: Vec<String>) -> String {
		let vars_handler = self.kak_vars_handler;
		let var_fifo_path = self.var_fifo_path;

    	let mut cmd = String::new();
    	// TODO: Need to check a type of a variable: %opt{} / %val{} / %arg{}
    	var_names.iter().for_each(|var| {
        	cmd.push_str(&format!("try %[ set -add global {vars_handler} {var} ] catch %[ set -add global {vars_handler} nil ] ;\n"));
    	});

    	cmd = format!("eval -client {client} %[
        declare-option str-list {vars_handler} ;
        set global {vars_handler}
        {cmd}
        echo -to-file {var_fifo_path} %opt[{vars_handler}] ;
    	]");

    	if self.debug {
        	println!("DEBUG::get_vars_vec() \n{cmd}");
    	}

    	talk_to_kak(session, cmd);

    	let var_fifo = OpenOptions::new()
        	.read(true)
        	.open(var_fifo_path)
        	.unwrap();
    	let mut var_reader = BufReader::new(var_fifo);
    	let mut vars = cmd.clear();
    	var_reader.read_to_string(&mut vars).unwrap();

    	vars
    }

}

fn create_fifo(path: &str, perm: u32) -> i32 {
	unsafe {
        let fifo_path = CString::new(path).unwrap().into_raw();
    	mkfifo(fifo_path as *const c_char, perm)
	}
}

fn talk_to_kak(session: &str, msg: String) {
	let mut k = Command::new("kak")
    	.args(["-p", session])
    	.stdin(Stdio::piped())
		.spawn()
		.unwrap();
	let mut k_stdin = k.stdin.take().unwrap();
	k_stdin.write_all(msg.as_bytes()).unwrap();
}

fn lua_exec(cmd: String) {
    let lua = mlua::Lua::new();
    if let Err(err) = lua.load(&cmd).set_name("Glua").unwrap().exec() {
        println!("Err: {}", err);
    };
}

fn read_glua(glua: Glua, client: &str, session: &str) {
    let main_fifo_path = glua.main_fifo_path;
    let vars_fifo_path = glua.vars_fifo_path;

	let main_fifo = OpenOptions::new()
    	.read(true)
    	.open(main_fifo_path)
    	.unwrap();

    let mut glua_main_reader = std::io::BufReader::new(main_fifo);
    let mut main_received = String::new();

    loop {
        glua_main_reader.read_to_string(&mut main_received).unwrap();

        if glua.debug {
            println!("DEBUG::run()::main_received\n {main_received}");
        }

        if main_received.starts_with("stop")  {
            break;
        }

        if main_received.starts_with("var") {
            main_received = main_received.replace("var", "");
			let var_val = get_var(var_fifo_path, client, session, vars_handler, main_received.trim(), false);
			println!("{var_val}");
        }

        if main_received.starts_with("Var") {
            main_received = main_received.replace("Var", "");
            let mut vars = main_received.split_whitespace().collect::<Vec<&str>>();
			let vars_val = get_vars_from_vec(var_fifo_path, client, session, vars_handler, vars, false);
			println!("{vars_val}");
        }

        main_received.clear();
        sleep(Duration::from_secs(1));
    }

}

fn main() {
 //    let _ = fs::remove_file(main_fifo_path);
	// let _ = fs::remove_file(vars_fifo_path);
}

