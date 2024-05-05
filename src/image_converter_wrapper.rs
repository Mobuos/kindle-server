use std::{io::ErrorKind, process::Command};

use rocket::Error;

// todo: do this better because I was just testing to see if it could work
pub fn convert(file: &str, background: &str) -> std::io::Result<()> {
    let output = Command::new("bash")
        // .arg("-c")
        .arg("./image-converter.sh")
        .arg("-b")
        .arg(background)
        .arg("-o")
        .arg(file)
        .output()
        .expect("Failed to get output");
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    if output.status.success() {
        Ok(())
    } else {
        // todo: this is not proper.. erroring here returns 500 on the rocket side
        let conversion_error = std::io::Error::new(ErrorKind::Other, "Failed conversion");
        Err(conversion_error)
    }
}
