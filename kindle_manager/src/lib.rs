use openssh::{KnownHosts, Session};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KindleManagerError {
    #[error("SSH error occurred: {0}")]
    SshError(#[from] openssh::Error),

    #[error("UTF-8 conversion failed: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("Command failed: {0}")]
    CommandError(String),
}
pub struct KindleManager {
    session: Session,
    location: String,
}

impl KindleManager {
    pub async fn new(address: String, location: String) -> Result<Self, KindleManagerError> {
        let session = Session::connect_mux(address, KnownHosts::Strict).await?;
        Ok(KindleManager { session, location })
    }

    pub async fn get_files(&self) -> Result<Vec<String>, KindleManagerError> {
        let output = self
            .session
            .command("ls")
            .arg(&self.location)
            .output()
            .await?;
        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;
        if !stderr.is_empty() {
            return Err(KindleManagerError::CommandError(stderr));
        }

        let files = stdout
            .split('\n')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(files)
    }
}

pub mod image_converter {}
