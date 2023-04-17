const QUOTES: [&str; 2] = ["¶", "§"];

enum KakArg {
    Opt(String),
    Val(String),
	Arg(String),
	Quote(String),
	Lit(String),
	Nop,
} 

struct Cmd {
    name: String,
    flags: Vec<String>,
    arguments: Vec<KakArg>,
}

// TODO: this is a trait function From or into ?
fn convert(kak_args: Vec<KakArg>, q_idx: usize) -> String {
    let q = QUOTES[q_idx];
    let mut cmd = String::new();
    let mut arg = String::new();
    for kak_arg in kak_args {
        match kak_arg {
            KakArg::Opt(a) => arg = format!("%opt{q}{a}{q} "),
            KakArg::Val(a) => arg = format!("%val{q}{a}{q} "),
            KakArg::Arg(a) => arg = format!("%a{q}{a}{q} "),
            KakArg::Quote(a) => arg = format!("%{q}{a}{q} "),
            KakArg::Lit(a) => arg = a,
            KakArg::Nop => String::new(),
        };
        cmd.push_str(&arg);
    }

    cmd
}


impl Cmd {
	fn new(name: String) -> Self {
    	Self {
        	name: name.to_string(),
        	flags: Vec::<String>::new(),
        	arguments: Vec::<KakArg>::new(),
    	}   
	}

	fn args(&mut self, args: Vec<KakArg>) -> &mut Self {
    	self.arguments = args.into();
    	self
	}

    fn arg(&mut self, arg: KakArg) -> &mut Self {
        self.arguments.push(arg);
        self
    }

    fn flag(&mut self, flag_name: String, flag_args: Vec<KakArg>) -> &mut Self {
        let args = convert(flag_args, 0);
        self.flags.push(format!("-{flag_name} {args}"));
        self
    }

    fn build(self) -> Self {
        self
    }
}

fn main() {
}
