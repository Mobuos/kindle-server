use std::{
    path::{Path, PathBuf},
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
}

pub mod image_converter {}
