use std::{
    path::Path,
    process::{Command, Output},
};

// TODO: Wrapper functions
// TODO: Check if already exists before attempting push or set

trait CheckStatus {
    fn check_status(self) -> Self;
}

// In case of failure, print stdout and stderr
impl CheckStatus for Output {
    fn check_status(self) -> Self {
        if !self.status.success() {
            println!(
                "stdout:\n{}",
                String::from_utf8(self.stdout.clone()).unwrap()
            );
            println!(
                "stderr:\n{}",
                String::from_utf8(self.stderr.clone()).unwrap()
            );
            panic!("Exit code {}", self.status)
        }
        self
    }
}

pub fn file_exists(filename: &str) -> bool {
    let check_output = Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("-g")
        .output()
        .expect("Failed to get existing files")
        .check_status();

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

    Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("--push")
        .arg(file)
        .output()
        .expect(format!("Failed to push {}!", filename).as_str())
        .check_status();

    if file_exists(filename) {
        println!("Pushed successfully!");
    } else {
        println!("Failed to push file...");
    }
}

// TODO: Appropriate error for when file doesn't exist
pub fn pull(filename: &str, target_location: &Path) {
    if !file_exists(filename) {
        println!("File doesn't exist! Not pulling");
        panic!();
    }

    Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("--pull")
        .arg(filename)
        .arg(target_location)
        .output()
        .expect(format!("Failed to pull {}!", filename).as_str())
        .check_status();
}

// Appropriate error for when file wasn't found
pub fn set(filename: &str) {
    if file_exists(filename) {
        Command::new("bash")
            .arg("./kindle-manager.sh")
            .arg("-a")
            .arg("kindle")
            .arg("--set")
            .arg(filename)
            .output()
            .expect(format!("Failed to set '{}'!", filename).as_str())
            .check_status();
    } else {
        panic!("File '{}' does not exist! Failed to set file!", filename);
    }
}

pub fn get_filenames() -> Vec<String> {
    let output = Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("--get-all")
        .output()
        .expect("Failed to get images on Kindle");

    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn delete_image(filename: &str) {
    if file_exists(filename) {
        Command::new("bash")
            .arg("./kindle-manager.sh")
            .arg("-a")
            .arg("kindle")
            .arg("--delete")
            .arg(filename)
            .output()
            .expect(format!("Failed to delete '{}'!", filename).as_str())
            .check_status();
    } else {
        println!("File {} not found!", filename);
    }
}

pub fn get_battery() -> String {
    let output = Command::new("bash")
        .arg("./kindle-manager.sh")
        .arg("-a")
        .arg("kindle")
        .arg("--battery")
        .output()
        .expect("Failed to get battery info from Kindle");

    String::from_utf8(output.stdout).unwrap()
}
