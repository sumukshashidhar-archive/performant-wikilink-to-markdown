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
use std::fs;
use rayon::prelude::*;

mod file_operations;
mod wikilink_operations;

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
    entries.par_iter().for_each(|entry| {
        match file_operations::read_file(&entry) {
            Ok(contents) => {
                let wikilinks = wikilink_operations::find_wikilinks(&contents, &entries, &entry);
                let output_file_path = file_operations::build_output_path(entry, input_path, output_path);
                match file_operations::write_output_file(&output_file_path, wikilinks.as_str()) {
                    Ok(_) => (),
                    Err(e) => println!("Error writing to output file {}: {}", output_file_path.display(), e),
                }
            }
            Err(e) => println!("Error reading file {}: {}", entry, e),
        }
    });
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
