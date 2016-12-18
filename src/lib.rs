use std::io::prelude::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::Result;
use std::path::PathBuf;


fn get_file_content(path: String) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn get_hash(text: String) -> u64 {
    let mut hasher = DefaultHasher::new();

    text.into_bytes().hash(&mut hasher);

    hasher.finish()
}

pub fn get_file_paths(dir: String) -> Result<Vec<PathBuf>> {
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

pub fn print_duplicats(duplicates: Vec<String>) {
    if duplicates.len() == 0 {
        println!("Duplicates not founded");
        return ();
    }

    println!("Duplicates ({}): ", duplicates.len());

    for path in duplicates {
        println!("{}", path);
    }
}

pub fn find_duplicates(files: Vec<PathBuf>) -> Vec<String> {
    let mut store = HashMap::new();
    let mut duplicates: Vec<String> = vec![];

    for file in &files {
        let file_path = file.to_str().unwrap();
        let file_content = get_file_content(file_path.to_string()).unwrap();
        let hash = get_hash(file_content);

        if !store.contains_key(&hash) {
            store.insert(hash, file_path);
        } else {
            duplicates.push(file_path.to_string());
        }
    }

    duplicates
}


#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::fs::{
        File,
        create_dir,
        remove_dir_all
    };
    use super::{
        get_file_content,
        get_hash
    };


    fn create_file(name: &'static str, text: &'static [u8; 9])  {
        let mut f = File::create(name.to_string()).unwrap();
        f.write_all(text).unwrap();
    }

    #[test]
    fn test_get_file_content() {
        create_dir("test-dir").unwrap();
        create_file("test-dir/test.txt", b"some text");

        let content = get_file_content("test-dir/test.txt".to_string()).unwrap();

        assert_eq!(content, "some text");
        remove_dir_all("test-dir").unwrap();
    }

    #[test]
    fn test_get_hash() {
        create_dir("test-dir").unwrap();
        create_file("test-dir/test.txt", b"some text");

        let content = get_file_content("test-dir/test.txt".to_string()).unwrap();
        let hash = get_hash(content.to_string());

        assert_eq!(hash, 17575663810583844296);
        remove_dir_all("test-dir").unwrap();
    }


}
