/*
File operations
 */

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{BufReader, Read, Write};


// Reads the contents of the file at the given path.
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

// Builds the output file path based on the input and output directories.
pub fn build_output_path(entry: &str, input_dir: &str, output_dir: &str) -> PathBuf {
    let input_path = Path::new(input_dir);
    let output_path = Path::new(output_dir);
    let relative_path = match Path::new(entry).strip_prefix(input_path) {
        Ok(path) => path,
        Err(_) => Path::new(entry),
    };
    
    output_path.join(relative_path)
}

// Writes the contents to the output file at the specified path.
pub fn write_output_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent_dir) = path.parent() {
    fs::create_dir_all(parent_dir)?;
    }
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}