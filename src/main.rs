use std::fs::{self, File};
use std::io::{BufReader, Read};

fn main() {
    let path = "test";
    let entries = get_entries(path);
    for entry in &entries {
        match read_file(&entry) {
            Ok(contents) => println!("{}: {}", entry, contents),
            Err(e) => println!("{}: {}", entry, e),
        }
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_entries(path: &str) -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry_result in read_dir {
            if let Ok(entry) = entry_result {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    entries.push(entry_path.to_str().unwrap().to_owned());
                } else if entry_path.is_dir() {
                    let subentries = get_entries(entry_path.to_str().unwrap());
                    entries.extend(subentries);
                }
            }
        }
    }
    entries
}
