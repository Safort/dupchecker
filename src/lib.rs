use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::io::Result;

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

pub fn get_file_paths(dir: String) -> Result<Vec<String>> {
    let dir_list = read_dir(dir)?;
    let mut file_paths = vec![];

    for dir in dir_list {
        let dir = dir?.path();
        let dir_str = dir.to_str().unwrap().to_string();

        if *&dir.is_file() {
            file_paths.push(dir_str);
        } else if *&dir.is_dir() {
            // wow! So ugly, such bad, much shit_code
            let mut dirs_paths = get_file_paths(dir_str).unwrap();

            file_paths.append(&mut dirs_paths);
        }
    }

    Ok(file_paths)
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

pub fn find_duplicates(file_paths: Vec<String>) -> Vec<String> {
    let mut store = HashMap::new();
    let mut duplicates: Vec<String> = vec![];

    for file_path in &file_paths {
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
    use super::{find_duplicates, get_file_data, get_file_paths, get_hash};
    use std;
    use std::fs::{create_dir, remove_dir_all, File};
    use std::io::prelude::*;
    use std::path::Path;

    fn create_file(name: &'static str, text: &str) {
        let mut f = File::create(name.to_string()).unwrap();
        f.write_all(text.as_bytes()).unwrap();
    }

    fn remove_dir(dir_path: &str) -> Result<(), std::io::Error> {
        if Path::new(dir_path).exists() {
            remove_dir_all(dir_path)
        } else {
            Ok(())
        }
    }

    fn create_new_dir(dir_path: &str) -> std::result::Result<(), std::io::Error> {
        remove_dir(dir_path)?;
        create_dir(dir_path)
    }

    #[test]
    fn test_get_file_data() {
        create_new_dir("test-dir").unwrap();
        create_file("test-dir/test.txt", "text");

        let content = get_file_data("test-dir/test.txt".to_string()).unwrap();

        assert_eq!(content, "text".to_string().into_bytes());
        remove_dir("test-dir").unwrap();
    }

    #[test]
    fn test_get_hash() {
        create_new_dir("test-dir-get-hash").unwrap();
        create_file("test-dir-get-hash/test.txt", "text");

        let content = get_file_data("test-dir-get-hash/test.txt".to_string()).unwrap();
        let hash = get_hash(content);

        assert_eq!(hash, 6797947405645866459);
        remove_dir("test-dir-get-hash").unwrap();
    }

    #[test]
    fn test_get_file_paths() {
        create_new_dir("test-dir-paths").unwrap();
        create_file("test-dir-paths/test1.txt", "text");
        create_file("test-dir-paths/test2.txt", "text");
        create_file("test-dir-paths/test3.txt", "text");

        let file_list = get_file_paths("test-dir-paths".to_string()).unwrap();

        assert_eq!(3, file_list.len());
        assert_eq!(
            true,
            file_list.contains(&"test-dir-paths/test1.txt".to_string())
        );
        assert_eq!(
            true,
            file_list.contains(&"test-dir-paths/test2.txt".to_string())
        );
        assert_eq!(
            true,
            file_list.contains(&"test-dir-paths/test3.txt".to_string())
        );

        remove_dir("test-dir-paths").unwrap();
    }

    #[test]
    fn test_find_duplicates() {
        create_new_dir("test-find-duplicates").unwrap();
        create_file("test-find-duplicates/test1.txt", "text");
        create_file("test-find-duplicates/test2.txt", "some text");
        create_file("test-find-duplicates/test3.txt", "text");
        create_file("test-find-duplicates/test4.txt", "another text");

        let file_list = get_file_paths("test-find-duplicates".to_string()).unwrap();
        let duplicates = find_duplicates(file_list);

        assert_eq!(1, duplicates.len());
        assert_eq!(
            true,
            duplicates.contains(&"test-find-duplicates/test3.txt".to_string())
        );

        remove_dir("test-find-duplicates").unwrap();
    }
}
