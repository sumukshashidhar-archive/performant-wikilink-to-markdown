
use std::path::{Path};
use pathdiff::diff_paths;
// Replaces wikilinks in the given text with their corresponding reference in entries.
pub fn find_wikilinks(text: &str, entries: &[String], entry: &str) -> String {
    let re = regex::Regex::new(r"\[\[(.+?)\]\]").unwrap();
    re.replace_all(text, |caps: &regex::Captures| {
        let reference = caps.get(1).unwrap().as_str();
        find_reference(reference, entries, entry)
    })
    .to_string()
}

// Finds the corresponding reference in entries and returns the properly formatted link.
pub fn find_reference(reference: &str, entries: &[String],  entry_path: &str) -> String {
    let matching_entry = entries.iter().find(|entry| {
        let filename = Path::new(entry).file_stem().unwrap().to_str().unwrap();
        filename == reference
    });

    match matching_entry {
        Some(entry) => format_link(entry, reference, entry_path),
        None => format!("[{}](./)", reference),
    }
}

// Formats the link using the given entry and reference.
pub fn format_link(entry: &str, reference: &str, entry_path: &str) -> String {

    let path_host = Path::new(entry_path);
    let path_reference = Path::new(entry);

    let rel_path = match diff_paths(path_reference, path_host) {
        Some(path) => path,
        None => {
            eprintln!("Error: Unable to compute relative path");
            return String::new();
        }
    };
    let rel_path = rel_path.strip_prefix("..").unwrap_or(&rel_path);
    let encoded_filename = rel_path.to_str().unwrap().replace(" ", "%20");
    format!("[{}]({})", reference, encoded_filename)
}