use std::io::prelude::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::{DefaultHasher};
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::Result;
use std::env;
use std::path::PathBuf;


fn get_file_content(path: String) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn get_file_hash(text: String) -> u64 {
    let mut hasher = DefaultHasher::new();

    text.into_bytes().hash(&mut hasher);

    hasher.finish()
}

fn get_file_paths(dir: String) -> Result<Vec<PathBuf>> {
    let dir_list = read_dir(dir)?;
    let mut files_paths = vec![];

    for item in dir_list {
        let i = item?.path();

        if *&i.is_file() {
            files_paths.push(i);
        }
    }

    Ok(files_paths)
}

fn print_duplicats(duplicates: Vec<String>) {
    if duplicates.len() == 0 {
        println!("Duplicates not founded");
        return ();
    }

    println!("Duplicates ({}): ", duplicates.len());

    for path in duplicates {
        println!("{}", path);
    }
}

fn find_duplicates(files: Vec<PathBuf>) -> Vec<String> {
    let mut store = HashMap::new();
    let mut duplicates: Vec<String> = vec![];

    for file in &files {
        let file_path = file.to_str().unwrap();
        let file_content = get_file_content(file_path.to_string()).unwrap();
        let hash = get_file_hash(file_content);

        if !store.contains_key(&hash) {
            store.insert(hash, file_path);
        } else {
            duplicates.push(file_path.to_string());
        }
    }

    duplicates
}


fn main() {
    let dir_path = env::args().nth(1).unwrap().to_string();
    let files = get_file_paths(dir_path).unwrap();
    let duplicates = find_duplicates(files);

    print_duplicats(duplicates);
}
