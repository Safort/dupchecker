use std::io::prelude::*;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::Result;
use std::path::PathBuf;


fn get_file_data(path: String) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn get_hash(file_data: Vec<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();

    file_data.hash(&mut hasher);

    hasher.finish()
}

pub fn get_file_paths(dir: String) -> Result<Vec<PathBuf>> {
    let dir_list = read_dir(dir)?;
    let mut files_paths = vec![];

    for item in dir_list {
        let i = item?.path();

        if *&i.is_file() {
            files_paths.push(i);
        } else if *&i.is_dir() {
            //wow! So ugly, such bad, much shit_code
            let mut dirs_paths = get_file_paths(i.to_str().unwrap().to_string()).unwrap();
            
            files_paths.append(&mut dirs_paths);
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
        let file_content = get_file_data(file_path.to_string()).unwrap();
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
        get_file_data,
        get_hash
    };


    fn create_file(name: &'static str, text: &'static [u8; 9])  {
        let mut f = File::create(name.to_string()).unwrap();
        f.write_all(text).unwrap();
    }

    #[test]
    fn test_get_file_data() {
        create_dir("test-dir").unwrap();
        create_file("test-dir/test.txt", b"some text");

        let content = get_file_data("test-dir/test.txt".to_string()).unwrap();

        assert_eq!(content, "some text".to_string().into_bytes());
        remove_dir_all("test-dir").unwrap();
    }

    #[test]
    fn test_get_hash() {
        create_dir("test-dir2").unwrap();
        create_file("test-dir2/test2.txt", b"some text");

        let content = get_file_data("test-dir2/test2.txt".to_string()).unwrap();
        let hash = get_hash(content);

        assert_eq!(hash, 17575663810583844296);
        remove_dir_all("test-dir2").unwrap();
    }
}
