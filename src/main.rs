extern crate dupchecker;

use std::env;
use dupchecker::{get_file_paths, find_duplicates, print_duplicats};

fn main() {
    let dir_path = env::args().nth(1).unwrap().to_string();
    let files = get_file_paths(dir_path).unwrap();
    let duplicates = find_duplicates(files);

    print_duplicats(duplicates);
}
