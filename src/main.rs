use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use urlencoding::encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: program_name <input_directory> <output_directory>");
        return;
    }
    let input_path = &args[1];
    let output_path = &args[2];
    let entries = get_entries(input_path);
    for entry in &entries {
        match read_file(&entry) {
            Ok(contents) => {
                let wikilinks = find_wikilinks(&contents, &entries);
                let output_path = get_output_path(entry, input_path, output_path);
                write_output_file(&output_path, wikilinks.as_str());
            }
            Err(e) => println!("Error reading file {}: {}", entry, e),
        }
    }
}

fn find_wikilinks(text: &str, entries: &Vec<String>) -> String {
    let re = regex::Regex::new(r"\[\[(.+?)\]\]").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let reference = caps.get(1).unwrap().as_str();
        find_reference(reference, entries)
    })
    .to_string()
}

fn find_reference(reference: &str, entries: &Vec<String>) -> String {
    let mut matching_entry: Option<&String> = None;
    for entry in entries {
        let filename = Path::new(entry).file_stem().unwrap().to_str().unwrap();
        if filename == reference {
            matching_entry = Some(entry);
            break;
        }
    }
    if let Some(entry) = matching_entry {
        let encoded_filename = encode(Path::new(entry).file_name().unwrap().to_str().unwrap());
        format!("[{}]({})", reference, encoded_filename)
    } else {
        format!("[{}](./)", reference)
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
    if let Err(_) = fs::read_dir(path) {
        return entries;
    }

    for entry_result in fs::read_dir(path).unwrap() {
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
    entries
}

fn get_output_path<'a>(path: &'a str, input_dir: &'a str, output_dir: &'a str) -> PathBuf {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);
    let relative_path = Path::new(path).strip_prefix(input_path).unwrap();
    let output_path = output_path.join(relative_path);
    output_path
}

fn write_output_file(path: &Path, contents: &str) -> std::io::Result<()> {
    fs::create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
