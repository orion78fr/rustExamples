use std::fs;
use ::config::Config;

pub fn search(conf: Config) {
    if conf.debug() {
        eprintln!("Looking for \"{}\" in file \"{}\"", conf.text(), conf.file());
    }

    let content = fs::read_to_string(conf.file()).expect("Cannot read file");
    //eprintln!("File content :\n\n{}", content);

    let matching_lines = search_in_string(conf.text(), &content);

    if conf.debug() {
        eprintln!("Got {} results", matching_lines.len());
    }

    for line in matching_lines {
        println!("{}", line);
    }
}

/// Returns all lines containing the text to search in the string
fn search_in_string<'a>(text_to_search: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    file_content.lines()
        .for_each(|line| {
            if line.contains(text_to_search) {
                result.push(line);
            }
        });

    return result;
}
