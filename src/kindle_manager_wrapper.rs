use std::{path::Path, process::Command};
// TODO: Wrapper functions
// TODO: Check if already exists before attempting push or set

pub fn file_exists(filename: &str) -> bool {
    let check_output = Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("-g")
        .output()
        .expect("Failed to get existing files");

    println!("kindle-manager: file_exists");
    let files = String::from_utf8(check_output.stdout).unwrap();
    println!("File to be checked: {}", filename);
    println!("{}", files);

    files.lines().any(|f| f == filename)
}

// TODO: Appropriate error for when file already exists
pub fn push(file: &Path) {
    let filename = file.file_name().unwrap().to_str().unwrap();
    if file_exists(filename) {
        println!("File already exists! Not pushing");
        panic!();
    }

    let _push_output = Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("--push")
        .arg(file)
        .output()
        .expect(format!("Failed to push {}!", filename).as_str());

    if file_exists(filename) {
        println!("Pushed successfully!");
    } else {
        println!("Failed to push file...");
    }
}

// Appropriate error for when file wasn't found
pub fn set(filename: &str) {
    if file_exists(filename) {
        let _set_output = Command::new("bash")
            .arg("./kindle-manager.sh")
            .arg("-a")
            .arg("kindle")
            .arg("--set")
            .arg(filename)
            .output()
            .expect(format!("Failed to set {}!", filename).as_str());
    }
    panic!("File does not exist! Failed to set file!");
}
