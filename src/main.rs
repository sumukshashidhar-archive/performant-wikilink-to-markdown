/**

Rust program for processing files by finding wikilinks and replacing them with their corresponding references.
Author: sumukshashidhar
Date: 2023-05-06
Usage: program_name <input_directory> <output_directory>
The program recursively reads all files in the input directory and its subdirectories,
processes each file by finding wikilinks and replacing them with their corresponding references,
and writes the processed files to the output directory with the same directory structure as the input directory.

*/

use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: program_name <input_directory> <output_directory>");
        return;
    }
    let input_path = &args[1];
    let output_path = &args[2];

    match get_entries(input_path) {
        Ok(entries) => process_entries(&entries, input_path, output_path),
        Err(e) => println!("Error reading input directory {}: {}", input_path, e),
    }
}

// Processes each entry by reading the file, finding wikilinks, and writing the output.
fn process_entries(entries: &[String], input_path: &str, output_path: &str) {
    for entry in entries {
        match read_file(&entry) {
            Ok(contents) => {
                let wikilinks = find_wikilinks(&contents, &entries);
                let output_file_path = build_output_path(entry, input_path, output_path);
                match write_output_file(&output_file_path, wikilinks.as_str()) {
                    Ok(_) => (),
                    Err(e) => println!("Error writing to output file {}: {}", output_file_path.display(), e),
                }
            }
            Err(e) => println!("Error reading file {}: {}", entry, e),
        }
    }
}


// Replaces wikilinks in the given text with their corresponding reference in entries.
fn find_wikilinks(text: &str, entries: &[String]) -> String {
    let re = regex::Regex::new(r"\[\[(.+?)\]\]").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let reference = caps.get(1).unwrap().as_str();
        find_reference(reference, entries)
    })
    .to_string()
}

// Finds the corresponding reference in entries and returns the properly formatted link.
fn find_reference(reference: &str, entries: &[String]) -> String {
    let matching_entry = entries.iter().find(|entry| {
        let filename = Path::new(entry).file_stem().unwrap().to_str().unwrap();
        filename == reference
    });

    match matching_entry {
        Some(entry) => format_link(entry, reference),
        None => format!("[{}](./)", reference),
    }
}

// Formats the link using the given entry and reference.
fn format_link(entry: &str, reference: &str) -> String {
    let path = PathBuf::from(entry);
    let filename = path.to_str().unwrap();
    let start_index = filename.find('/').and_then(|i| filename[i + 1..].find('/').map(|j| i + j + 1)).unwrap_or(0);
    let encoded_filename = &filename[start_index..].replace(" ", "%20");
    format!("[{}](.{})", reference, encoded_filename)
}

// Reads the contents of the file at the given path.
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

// Recursively retrieves file entries in the specified directory.
fn get_entries(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut entries = Vec::new();

    for entry_result in fs::read_dir(path)? {
        if let Ok(entry) = entry_result {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if let Some(path_str) = entry_path.to_str() {
                    entries.push(path_str.to_owned());
                }
            } else if entry_path.is_dir() {
                let subentries = get_entries(entry_path.to_str().unwrap())?;
                entries.extend(subentries);
            }
        }
    }

    Ok(entries)
}


// Builds the output file path based on the input and output directories.
fn build_output_path(entry: &str, input_dir: &str, output_dir: &str) -> PathBuf {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);
    let relative_path = match Path::new(entry).strip_prefix(input_path) {
        Ok(path) => path,
        Err(_) => Path::new(entry),
    };
    
    output_path.join(relative_path)
}

// Writes the contents to the output file at the specified path.
fn write_output_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent_dir) = path.parent() {
    fs::create_dir_all(parent_dir)?;
    }
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

