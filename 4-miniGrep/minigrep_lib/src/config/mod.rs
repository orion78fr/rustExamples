pub struct Config<'a> {
    text: &'a str,
    file: &'a str,
}

impl<'a> Config<'a> {
    pub fn text(&self) -> &str {
        self.text
    }
    pub fn file(&self) -> &str {
        self.file
    }
}

pub fn parse_args<'a>(args: &'a Vec<String>) -> Result<Config<'a>, String> {
    //eprintln!("Called with : {:?}", args);

    if args.len() != 3 {
        show_usage(&args[0]);
        return Err(format!("Wrong number of arguments : expected 3, got 2"));
    }

    return Ok(Config {
        text: &args[1],
        file: &args[2],
    });
}

fn show_usage(executable_name: &str) {
    eprintln!("Usage : {} <str_to_search> <file>", executable_name);
}
