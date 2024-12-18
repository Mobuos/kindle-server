use std::{
    path::Path,
    process::{Command, Output},
    u8,
};

use openssh::{KnownHosts, Session};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KindleManagerError {
    #[error("SSH error occurred: {0}")]
    SshError(#[from] openssh::Error),

    #[error("UTF-8 conversion failed: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("IO error occurred: {0}")]
    StdioError(#[from] std::io::Error),

    #[error("Command failed: {0}")]
    CommandError(String),

    #[error("Argument out of allowed range: {0}")]
    OutOfRange(String),

    #[error("Command failed, file already exists: {0}")]
    FileExists(String),

    #[error("Command failed, file doesn't exist: {0}")]
    FileMissing(String),
}

#[derive(Debug)]
pub struct KindleManager {
    address: String,
    location: String,
}

trait CheckStdout {
    fn check_stdout(self) -> Result<String, KindleManagerError>;
}

impl CheckStdout for Output {
    /// Checks for output status, returning Ok(stdout) or stderr
    fn check_stdout(self) -> Result<String, KindleManagerError> {
        let stdout = String::from_utf8(self.stdout)?;
        let stderr = String::from_utf8(self.stderr)?;

        if self.status.success() {
            Ok(stdout)
        } else {
            Err(KindleManagerError::CommandError(stderr))
        }
    }
}

impl KindleManager {
    pub fn new(address: String, location: String) -> Self {
        KindleManager { address, location }
    }

    pub async fn new_session(&self) -> Result<Session, KindleManagerError> {
        Ok(Session::connect_mux(&self.address, KnownHosts::Strict).await?)
    }

    pub async fn debug_print(
        &self,
        session: &Session,
        text: &str,
    ) -> Result<(), KindleManagerError> {
        let _ = session
            .command("fbink")
            .arg("-q")
            .arg(text)
            .args(["-x", "1"])
            .args(["-y", "2"])
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    // Credit to https://github.com/mattzzw/kindle-clock
    /// Prepares the Kindle to act as a display, disabling services to save power,
    /// entering power-saving mode and disabling the screen-saver.
    pub async fn prep(&self, session: &Session) -> Result<(), KindleManagerError> {
        // TODO: Check if we can stop framework and powerd
        let services_to_stop = ["lab126_gui", "otaupd", "phd", "tmd", "x", "todo"];
        for service in services_to_stop {
            self.stop_service(&session, service).await?;
        }

        // Set lowest CPU clock
        let _ = session
            .command("sh")
            .arg("-c")
            .arg("echo powersave > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
            .output()
            .await?
            .check_stdout()?;

        // Disable Screensaver
        let _ = session
            .command("sh")
            .arg("-c")
            .arg("lipc-set-prop com.lab126.powerd preventScreenSaver 1")
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    async fn stop_service(
        &self,
        session: &Session,
        service: &str,
    ) -> Result<(), KindleManagerError> {
        let _ = session
            .command("stop")
            .arg(service)
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    pub async fn list_files(&self, session: &Session) -> Result<Vec<String>, KindleManagerError> {
        let stdout = session
            .command("ls")
            .arg(&self.location)
            .output()
            .await?
            .check_stdout()?;

        let files = stdout
            .split('\n')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(files)
    }

    pub async fn file_exists(
        &self,
        session: &Session,
        kindle_filename: &str,
    ) -> Result<bool, KindleManagerError> {
        return Ok(self
            .list_files(session)
            .await?
            .iter()
            .any(|filename| filename == kindle_filename));
    }

    pub async fn push_file(
        &self,
        session: &Session,
        local_file_path: &Path,
        kindle_filename: &str,
    ) -> Result<(), KindleManagerError> {
        if self.file_exists(&session, kindle_filename).await? {
            return Err(KindleManagerError::FileExists(format!("{kindle_filename}")));
        }

        let _ = Command::new("scp")
            .arg(local_file_path)
            .arg(format!(
                "{}:{}/{}",
                self.address, self.location, kindle_filename
            ))
            .output()?
            .check_stdout()?;

        Ok(())
    }

    pub async fn pull_file(
        &self,
        session: &Session,
        kindle_filename: &str,
        local_file_path: &Path,
    ) -> Result<(), KindleManagerError> {
        if !self.file_exists(&session, kindle_filename).await? {
            return Err(KindleManagerError::FileMissing(format!(
                "{kindle_filename}"
            )));
        }

        let _ = Command::new("scp")
            .arg(format!(
                "{}:{}/{}",
                self.address, self.location, kindle_filename
            ))
            .arg(local_file_path)
            .output()?
            .check_stdout()?;

        Ok(())
    }

    pub async fn rename_file(
        &self,
        session: &Session,
        old_filename: &str,
        new_filename: &str,
    ) -> Result<(), KindleManagerError> {
        if !self.file_exists(&session, old_filename).await? {
            return Err(KindleManagerError::FileMissing(format!("{old_filename}")));
        }
        if self.file_exists(&session, new_filename).await? {
            return Err(KindleManagerError::FileExists(format!("{new_filename}")));
        }

        let _ = session
            .command("mv")
            .arg(format!("{}/{}", self.location, old_filename))
            .arg(format!("{}/{}", self.location, new_filename))
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    pub async fn delete_file(
        &self,
        session: &Session,
        kindle_filename: &str,
    ) -> Result<(), KindleManagerError> {
        if !self.file_exists(&session, kindle_filename).await? {
            return Err(KindleManagerError::FileMissing(format!(
                "{kindle_filename}"
            )));
        }

        let _ = session
            .command("rm")
            .arg(format!("{}/{}", self.location, kindle_filename))
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    pub async fn set_image(
        &self,
        session: &Session,
        filename: &str,
    ) -> Result<(), KindleManagerError> {
        if !self.file_exists(&session, filename).await? {
            return Err(KindleManagerError::FileMissing(format!("{filename}")));
        }

        let _ = session
            .command("sh")
            .arg("-c")
            .arg(format!(
                "eips -c; eips -f; eips -g \"{}/{}\"",
                self.location, filename
            ))
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    pub async fn battery_charge(&self, session: &Session) -> Result<u8, KindleManagerError> {
        let stdout = session
            .command("gasgauge-info")
            .arg("-c")
            .output()
            .await?
            .check_stdout()?;

        let stdout: String = stdout.chars().filter(|c| c.is_digit(10)).collect();
        match stdout.parse::<u8>() {
            Ok(battery) => Ok(battery),
            Err(err) => Err(KindleManagerError::CommandError(format!(
                "Failed conversion of {stdout}: {err}"
            ))),
        }
    }

    pub async fn battery_load(&self, session: &Session) -> Result<String, KindleManagerError> {
        let stdout = session
            .command("gasgauge-info")
            .arg("-l")
            .output()
            .await?
            .check_stdout()?;

        Ok(stdout)
    }

    pub async fn set_backlight(
        &self,
        session: &Session,
        intensity: u8,
    ) -> Result<(), KindleManagerError> {
        // Intensity seems to be between 0..=255
        // Higher values don't do anything more
        let _ = session
            .command("sh")
            .arg("-c")
            .arg(format!(
                "echo -n {} > /sys/devices/system/fl_tps6116x/fl_tps6116x0/fl_intensity",
                intensity
            ))
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }
}

pub mod image_converter {
    use std::{path::PathBuf, process::Command};

    use crate::{CheckStdout, KindleManagerError};

    // TODO: Check if the raster library can replace this
    pub fn convert_image(
        background: &str,
        stretch: bool,
        origin: &PathBuf,
        destination: &PathBuf,
    ) -> Result<(), KindleManagerError> {
        let mut resize = "";
        if stretch {
            resize = "!";
        }
        let stdout = Command::new("magick")
            .arg(origin)
            .args([
                "-filter",
                "LanczosSharp",
                "-resize",
                &format!("758x1024{}", resize),
                "-background",
                background,
                "-gravity",
                "center",
                "-extent",
                "758x1024",
                "-colorspace",
                "Gray",
                "-dither",
                "FloydSteinberg",
                "-remap",
                "kindle_colors.gif", // TODO: Include this in the final binary somehow? https://stackoverflow.com/questions/76252932/creating-palette-image-on-the-fly-for-remap-behind-palette-is-not-reset-each-t
                "-quality",
                "75",
                "-define",
                "png:color-type=0",
                "-define",
                "png:bit-depth=8",
            ])
            .arg(destination)
            .output()?
            .check_stdout()?;

        println!("{stdout}");

        Ok(())
    }
}
