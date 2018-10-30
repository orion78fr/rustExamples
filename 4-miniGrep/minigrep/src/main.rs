extern crate minigrep_lib;

use std::env;
use minigrep_lib::searcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Called with : {:?}", args);
        eprintln!("Usage : {} <str_to_search> <file>", &args[0]);
        std::process::exit(-1);
    }

    let text = &args[1];
    let file = &args[2];

    searcher::search(text, file);
}
