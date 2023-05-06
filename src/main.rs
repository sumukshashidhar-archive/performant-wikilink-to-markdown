use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    match read_file("data/a.md") {
        Ok(contents) => println!("{}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
