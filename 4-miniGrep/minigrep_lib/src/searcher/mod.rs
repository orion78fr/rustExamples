use config::Config;
use std::{fs, io::Read};

pub fn search(conf: Config) {
    if conf.debug() {
        eprintln!("Looking in files {:?}", conf.files());
    }

    if conf.standard_input() {
        let mut content = String::new();
        std::io::stdin().read_to_string(&mut content)
            .expect(&format!("Error while reading standard input"));

        let matching_lines = search_in_string(&conf, &content);

        if conf.debug() {
            eprintln!("Got {} results in standard input", matching_lines.len());
        }
        for line in matching_lines {
            println!("{}", line);
        }
    }

    conf.files().iter()
        .for_each(|f| {
            let content = fs::read_to_string(f)
                .expect(&format!("Cannot read file {}", f));

            // eprintln!("File content :\n\n{}", content);
            let matching_lines = search_in_string(&conf, &content);

            if conf.debug() {
                eprintln!("Got {} results in file {}", matching_lines.len(), f);
            }
            for line in matching_lines {
                println!("{}", line);
            }
        })
}

/// Returns all lines containing the text to search in the string
fn search_in_string<'a>(conf: &Config, file_content: &'a str) -> Vec<&'a str> {
    return file_content.lines()
        .filter(|line| conf.match_text(line))
        .collect();
}
