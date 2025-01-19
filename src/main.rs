extern crate dupchecker;

use dupchecker::{find_duplicates, get_file_paths, print_duplicats};
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let dir_path = if args.len() == 0 {
        current_dir()?
    } else {
        PathBuf::from(args.nth(1).unwrap())
    };

    let files = get_file_paths(dir_path)?;
    let duplicates = find_duplicates(files);

    print_duplicats(duplicates);

    Ok(())
}
