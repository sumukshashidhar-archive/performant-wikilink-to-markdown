use std::fs;

fn main() {
    let path = "data";
    let entries = get_entries(path);
    for entry in entries {
        println!("{}", entry);
    }
}

fn get_entries(path: &str) -> Vec<String> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry_result in read_dir {
            if let Ok(entry) = entry_result {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    entries.push(entry_path.to_str().unwrap().to_owned());
                    let subentries = get_entries(entry_path.to_str().unwrap());
                    entries.extend(subentries);
                } else if entry_path.is_file() {
                    entries.push(entry_path.to_str().unwrap().to_owned());
                }
            }
        }
    }
    entries
}
