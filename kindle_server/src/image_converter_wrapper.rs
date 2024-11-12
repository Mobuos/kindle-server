use std::{io::ErrorKind, process::Command};

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
    println!("image_converter: convert:");
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    if output.status.success() {
        Ok(())
    } else {
        let conversion_error = std::io::Error::new(ErrorKind::Other, "Failed conversion");
        Err(conversion_error)
    }
}
