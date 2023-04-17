use std::fmt::Display;

enum KakArg<T> {
    Opt(T),
    Val(T),
    Arg(T),
    Quote(T),
    Lit(T),
    Nop,
}

use KakArg::*;

impl<T: Display + ToString> KakArg<T> {
    fn to_string(&self) -> String {
        let q = "¶";
        match self {
            Opt(a) => format!("%opt{q}{a}{q}"),
            Val(a) => format!("%val{q}{a}{q}"),
            Arg(a) => format!("%a{q}{a}{q}"),
            Quote(a) => format!("%{q}{a}{q}"),
            Lit(a) => a.to_string(),
            Nop => String::new(),
        }
    }
}

struct CmdBuilder {
    command: Vec<String>,
    arguments: Vec<String>,
}

impl CmdBuilder {
    fn new<T: ToString>(name: T) -> Self {
        Self {
            command: vec![name.to_string()],
            arguments: Vec::<String>::new(),
        }
    }

    fn arg<T: ToString + Display>(&mut self, arg: KakArg<T>) -> &mut Self {
        let arg = arg.to_string();

        if !arg.is_empty() {
            self.arguments.push(arg);
        }

        self
    }

    fn flag<T: ToString + Display>(&mut self, flag_name: T, flag_args: KakArg<T>) -> &mut Self {
        let mut flag = flag_name.to_string();
        let args = flag_args.to_string();

        if !flag.starts_with('-') {
            flag.insert(0, '-')
        }

        if !flag.is_empty() {
            self.command.push(flag);
        }

        if !args.is_empty() {
            self.command.push(args);
        }

        self
    }

    fn build(&mut self) -> String {
        self.command.append(&mut self.arguments);

        self.command.join(" ")
    }
}
