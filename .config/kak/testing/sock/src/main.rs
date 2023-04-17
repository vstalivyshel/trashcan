mod cmd;
use crate::cmd::*;
use std::ffi::CString;
use std::fs::{self, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

extern "C" {
    fn mkfifo(path: *const i8, permission: u32) -> i32;
}


fn create_fifo(path: &str, perm: u32) -> i32 {
    unsafe {
        let fifo_path = CString::new(path).unwrap().into_raw();
        mkfifo(fifo_path as *const i8, perm)
    }
}

fn opt_quote(string: &str) -> String {
	format!("%opt{q}{string}{q}", q = OPT_Q)
}

fn kak_send_cmd(target: State, cmd: &str) {
    let mut k = Command::new("kak")
        .args(["-p", session])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut k_stdin = k.stdin.take().unwrap();
    k_stdin.write_all(cmd.as_bytes()).unwrap();
}


enum Kak {
    Debug(String),
    Error(String),
}

impl Kak {
    fn eval(flags: &str, cmd: &str) -> String {
		format!("eval {flags} {cmd}", cmd = quote(cmd))
    }

    fn echo(flags:&str, msg: &str) -> String {
        format!("echo {flags} {msg}", msg = quote(msg))
    }

    fn set(flags: &str, scope: &str, name: &str, val: &str) -> String {
        format!("set {flags} {scope} {name} {val}", val = quote(val))
    }
    fn try(cmd: &str, catch_cmd: &str) -> String {
        format!("try {cmd} catch {catch_cmd}", cmd = quote(cmd), catch_cmd = quote(catch_cmd))
    }

    fn get_vars(target: &State, var_names: Vec<&str>) -> String {
        let client = &target.client;
        let session = &target.session;
        let vars_handler = &target.vars_handler;
        let var_fifo_path = &target.vars_fifo_path;

        // TODO: Need to check a type of a variable: %opt{} / %val{} / %arg{}
        let mut cmd = String::new();
        let g = "global";
        let a = "-add";
        let or_nil = &Kak::set(a, g, vars_handler, "nil");
        var_names.iter().for_each(|var| {
            let get = &Kak::set(a, g, vars_handler, var);
            let try_get = &Kak::try(get, or_nil);
        	cmd.push_str(try_get);
    	});
    	let to_file = format!("-to-file {var_fifo_path}");
    	let vars = quote(vars_handler);
    	let echo = &Kak::echo(to_file, opt_quote(vars_handler));
    	cmd.push_str()
		let client = format!("-try-client {client}");
		cmd = Kak::eval(client, cmd)
        cmd = format!(
        "eval -client {client} %[\
        declare-option str-list {vars_handler} ;\
        set global {vars_handler}\
        {cmd}\
        echo -to-file {var_fifo_path} %opt[{vars_handler}] ;\
    	]"
        );

        if self.debug {
            println!("DEBUG::State::get_vars() >>>\n{cmd}\n");
        }

        self.send_cmd(&cmd);

        let var_fifo = OpenOptions::new().read(true).open(var_fifo_path).unwrap();
        let mut var_reader = BufReader::new(var_fifo);

        cmd.clear();
        let mut vars = cmd;
        var_reader.read_to_string(&mut vars).unwrap();

        vars
    }
}

    // fn send_err(&self, status_msg: &str, err_msg: &str) {
    //     let client = &self.client;
    //     let session = &self.session;
    //     let d = ;
    //     let cmd = format!("eval -client {client} %[
    //     echo -markup {{Error}}{status_msg} ;
    //     echo -debug %[{err_msg}]
    //     ]");
    //     self.send_cmd(&cmd);
    // }

struct State {
    session: String,
    client: String,
    main_fifo_path: String,
    vars_fifo_path: String,
    vars_handler: String,
    debug: bool,
}

impl State {
    fn new(session: &str, glua_temp_dir: &str) -> Self {
        let glua_temp_dir = glua_temp_dir.to_string();

        let session = session.to_string();
        let client = "client0".to_string();

        let mut main_fifo_path = glua_temp_dir.clone();
        main_fifo_path.push_str("/glua_main_fifo");

        let mut vars_fifo_path = glua_temp_dir;
        vars_fifo_path.push_str("/glua_vars_fifo");

        let vars_handler = "kak_glua_vars".to_string();
        let debug = false;

        Self {
            session,
            client,
            main_fifo_path,
            vars_fifo_path,
            vars_handler,
            debug,
        }
    }

    fn debug(&mut self, enable: bool) {
        self.debug = enable;
    }

    fn set_client(&mut self, client_name: &str) {
        self.client = client_name.to_string();
    }

    fn set_session(&mut self, session: &str) {
		self.session = session.to_string();
    }


    // TODO: How to work with Kak obj function in Lua obj?
    fn lua_eval(&self, chunk: String) {
        let client =self.client.clone();
        let session = self.session.clone();

        let lua = mlua::Lua::new();
        let globals = lua.globals();

        let echo = lua.create_function(|_, msg: String| Ok(self.echo(&msg)) ).unwrap();

        let kak_globals = lua.create_table().unwrap();
        kak_globals.set("echo", echo).unwrap();
        globals.set("kak", kak_globals).unwrap();

        if let Err(err) = lua.load(&chunk).set_name("Glua::State").unwrap().exec() {
            if self.debug {
                println!("DEBUG::State::lua_eval::Err >>> \n{}", err);
            }
            // self.send_err("Glua::Err::lua_eval()", &err);
        };
    }
}

#[derive(Debug)]
enum Do {
    GetVars,
    SendErr,
    LuaEval,
    Stop,
    Nop,
}

fn understand(chunk: &str) -> Do {
	match chunk {
		"%%V" => Do::GetVars,
		"%%E" => Do::SendErr,
		"%%L" => Do::LuaEval,
		"%%S" => Do::Stop,
		_ => Do::Nop
	}
}

fn run() {
    let mut kak = State::new("sock", "./");
    kak.debug(true);
    let main_fifo_path = &kak.main_fifo_path;
    let vars_fifo_path = &kak.vars_fifo_path;

    create_fifo(main_fifo_path, 444);
    create_fifo(vars_fifo_path, 444);

    let main_fifo = OpenOptions::new().read(true).open(main_fifo_path).unwrap();

    let mut main_received = String::new();
    let mut glua_main_reader = std::io::BufReader::new(main_fifo);

    loop {
        glua_main_reader.read_to_string(&mut main_received).unwrap();

		if main_received.starts_with("%%") {
    		let (prefix, chunk) = main_received.split_at(3);
			let action = understand(&prefix);

    		if kak.debug {
        		println!("DEBUG::run() action = {action:?} ; prefix = \"{prefix}\" ; main_received >>>\n{chunk}")
    		}

    		match action {
        		Do::GetVars => {
                    let vars = chunk.split_whitespace().collect::<Vec<&str>>();
                    let vars_val = kak.get_vars(vars);
                    println!("{vars_val}");
        		}
        		Do::SendErr =>  kak.send_err("Glua::ManualError", chunk),
        		Do::LuaEval =>  kak.lua_eval(chunk.to_string()),
        		Do::Stop => break,
        		Do::Nop => println!("{chunk}"),
    		}
		} else if kak.debug && !main_received.is_empty() {
            println!("DEBUG::run()::main_received >>> {main_received}");
        }

        main_received.clear();
        sleep(Duration::from_millis(200));
    }

    let _ = fs::remove_file(main_fifo_path);
    let _ = fs::remove_file(vars_fifo_path);
}

fn main() {
    run()
}
