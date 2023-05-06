use std::process::Command;

#[test]
fn test_program() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("./tests/candidate")
        .arg("./tests/output")
        .output()
        .expect("failed to run program");

    assert!(output.status.success(), "program exited with an error: {:?}", output);
}