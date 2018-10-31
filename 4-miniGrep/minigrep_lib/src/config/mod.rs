use std::env;

pub struct Config<'a> {
    text: &'a str,
    file: &'a str,
    debug: bool
}

impl<'a> Config<'a> {
    pub fn text(&self) -> &str {
        self.text
    }
    pub fn file(&self) -> &str {
        self.file
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
}

pub fn parse_args<'a>(args: &'a Vec<String>) -> Result<Config<'a>, String> {
    let debug = is_debug_enabled();

    if debug {
        eprintln!("Called with : {:?}", args);
    }

    if args.len() != 3 {
        show_usage(&args[0]);
        return Err(format!("Wrong number of arguments : expected 3, got 2"));
    }

    let text = &args[1];
    let file = &args[2];

    return Ok(Config {
        text,
        file,
        debug,
    });
}

fn show_usage(executable_name: &str) {
    eprintln!("Usage : {} <str_to_search> <file>", executable_name);
}

fn is_debug_enabled() -> bool {
    match env::var("DEBUG") {
        Ok(v) => v.eq("true") || v.eq("y"),
        Err(_) => false
    }
}
