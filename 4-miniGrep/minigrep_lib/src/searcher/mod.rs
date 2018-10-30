use std::fs;
use ::config::Config;

pub fn search(conf: Config) {
    eprintln!("Looking for \"{}\" in file \"{}\"", conf.text(), conf.file());

    let content = fs::read_to_string(conf.file()).expect("Cannot read file");
    //eprintln!("File content :\n\n{}", content);

    search_in_string(conf.text(), content);
}

fn search_in_string(text_to_search: &str, file_content: String) {
    file_content.lines()
        .for_each(|line| {
            if line.contains(text_to_search) {
                println!("{}", line);
            }
        });
}