pub struct Config {
    text: String,
    file: String,
}

impl Config {
    pub fn text(&self) -> String {
        self.text.clone()
    }
    pub fn file(&self) -> String {
        self.file.clone()
    }
}

pub fn parse_args(args: Vec<String>) -> Result<Config, String> {
    //eprintln!("Called with : {:?}", args);

    if args.len() != 3 {
        show_usage(&args[0]);
        return Err(format!("Wrong number of arguments : expected 3, got 2"));
    }

    return Ok(Config {
        text: args[1].clone(),
        file: args[2].clone(),
    });
}

fn show_usage(executable_name: &String) {
    eprintln!("Usage : {} <str_to_search> <file>", executable_name);
}
