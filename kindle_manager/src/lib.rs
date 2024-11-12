use openssh::{KnownHosts, Session};

pub struct KindleManager {
    session: Session,
    location: String,
}

impl KindleManager {
    pub async fn new(address: String, location: String) -> Result<Self, openssh::Error> {
        let session = Session::connect_mux(address, KnownHosts::Strict).await?;
        Ok(KindleManager { session, location })
    }

    pub async fn get_files(&self) -> Result<Vec<String>, openssh::Error> {
        let output = self
            .session
            .command("ls")
            .arg(&self.location)
            .output()
            .await?;
        let stdout = String::from_utf8(output.stdout).expect("Server output was not valid UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("Error output was not valid UTF-8");
        if !stderr.is_empty() {
            eprintln!("Kindle Manager Error: {}", stderr);
        }

        Ok(stdout
            .split('\n')
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }
}

pub mod image_converter {}
