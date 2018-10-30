extern crate minigrep_lib;

use std::env;
use minigrep_lib::{searcher, config::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_args(args).expect("Problem during args parsing");

    searcher::search(config);
}