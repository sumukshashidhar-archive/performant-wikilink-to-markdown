use std::process::Command;
use std::collections::HashSet;
use walkdir::WalkDir;


#[test]
fn test_program() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("./tests/candidate")
        .arg("./tests/output")
        .output()
        .expect("failed to run program");

    assert!(output.status.success(), "program exited with an error: {:?}", output);

    let ideal_dir = "./tests/ideal";
    let output_dir = "./tests/output";

    // Collect the filenames in ideal directory
    let ideal_filenames: HashSet<String> = WalkDir::new(ideal_dir)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    // Collect the filenames in output directory
    let output_filenames: HashSet<String> = WalkDir::new(output_dir)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    // Check if the filename sets are equal
    assert_eq!(ideal_filenames, output_filenames, "The output directory does not match the ideal directory");
}