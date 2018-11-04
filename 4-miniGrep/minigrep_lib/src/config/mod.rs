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
    pub fn standard_input(&self) -> bool { self.standard_input }
}

pub fn parse_args<'a>(args: &'a Vec<String>) -> Result<Config<'a>, String> {
    let debug = is_debug_enabled();

    if debug {
        eprintln!("Called with : {:?}", args);
    }

    let mut inverted = false;
    let mut regexp = false;
    let mut text_to_match: Option<&str> = None;
    let mut files = Vec::new();

    for i in 1..args.len() {
        match args[i].as_str() {
            "-v" | "--inverted" => inverted = true,
            "-g" | "--regexp" => regexp = true,
            txt => match text_to_match {
                None => text_to_match = Some(txt),
                Some(_) => files.push(txt)
            }
        }
    }

    let mut matcher: Box<Matcher + 'a>;

    match text_to_match {
        None => {
            show_usage(&args[0]);
            return Err("Missing argument".to_string());
        }
        Some(txt) => {
            if regexp {
                matcher = Box::new(Matcher::of_regex(txt))
            } else {
                matcher = Box::new(Matcher::of_text(txt))
            }
        }
    }

    if inverted {
        matcher = Box::new(Matcher::not(matcher))
    }

    let standard_input = files.is_empty();

    return Ok(Config {
        matcher,
        files,
        debug,
        standard_input,
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
