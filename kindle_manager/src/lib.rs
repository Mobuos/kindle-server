use std::{
    path::Path,
    process::{Command, Output},
};

use openssh::{KnownHosts, Session};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KindleManagerError {
    #[error("SSH error occurred: {0}")]
    SshError(#[from] openssh::Error),

    #[error("UTF-8 conversion failed: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("IO error occured: {0}")]
    StdioError(#[from] std::io::Error),

    #[error("Command failed: {0}")]
    CommandError(String),
}
pub struct KindleManager {
    address: String,
    session: Session,
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
    pub async fn new(address: String, location: String) -> Result<Self, KindleManagerError> {
        // Create an openSSH session
        let session = Session::connect_mux(&address, KnownHosts::Strict).await?;

        Ok(KindleManager {
            address,
            session,
            location,
        })
    }

    pub async fn debug_print(&self, text: &str) -> Result<(), KindleManagerError> {
        let _ = self
            .session
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
    pub async fn prep(&self) -> Result<(), KindleManagerError> {
        // TODO: Check if we can stop framework and powerd
        let services_to_stop = ["lab126_gui", "otaupd", "phd", "tmd", "x", "todo"];
        for service in services_to_stop {
            self.stop_service(service).await?;
        }

        // Set lowest CPU clock
        let _ = self
            .session
            .command("sh")
            .arg("-c")
            .arg("echo powersave > /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor")
            .output()
            .await?
            .check_stdout()?;

        // Disable Screensaver
        let _ = self
            .session
            .command("sh")
            .arg("-c")
            .arg("lipc-set-prop com.lab126.powerd preventScreenSaver 1")
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    async fn stop_service(&self, service: &str) -> Result<(), KindleManagerError> {
        let _ = self
            .session
            .command("stop")
            .arg(service)
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    pub async fn list_files(&self) -> Result<Vec<String>, KindleManagerError> {
        let stdout = self
            .session
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

    pub async fn push_file(
        &self,
        local_file_path: &Path,
        kindle_filename: &str,
    ) -> Result<(), KindleManagerError> {
        self.session.check().await?;
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
        kindle_filename: &str,
        local_file_path: &Path,
    ) -> Result<(), KindleManagerError> {
        self.session.check().await?;
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

    pub async fn delete_file(&self, kindle_filename: &str) -> Result<(), KindleManagerError> {
        let _ = self
            .session
            .command("rm")
            .arg(format!("{}/{}", self.location, kindle_filename))
            .output()
            .await?
            .check_stdout()?;

        Ok(())
    }

    pub async fn set_image(&self, filename: &str) -> Result<(), KindleManagerError> {
        let _ = self
            .session
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

    pub async fn info_battery(&self) -> Result<u8, KindleManagerError> {
        let stdout = self
            .session
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
}

pub mod image_converter {}
