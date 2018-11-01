use std::env;
use matcher::Matcher;

pub struct Config<'a> {
    matcher: Box<Matcher + 'a>,
    files: Vec<&'a str>,
    standard_input: bool,
    debug: bool,
}

impl<'a> Config<'a> {
    pub fn match_text(&self, text_to_test: &str) -> bool {
        self.matcher.match_text(text_to_test)
    }
    pub fn files(&self) -> &Vec<&'a str> {
        &self.files
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

    let matcher = Matcher::of_text(&args[1]);

    let mut files = Vec::new();
    files.push(args[2].as_str());

    return Ok(Config {
        matcher: Box::new(matcher),
        files,
        debug,
        standard_input: false,
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
