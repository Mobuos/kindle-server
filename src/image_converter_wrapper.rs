use std::process::Command;

// todo: Create enum for background
// todo: do this better because I was just testing to see if it could work
pub fn convert(file: &str, background: &str) {
    let output = Command::new("bash")
        // .arg("-c")
        .arg("./image-converter.sh")
        .arg("-b")
        .arg(background)
        .arg("-o")
        .arg(file)
        .output()
        .expect("Failed");
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}
