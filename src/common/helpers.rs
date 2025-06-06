use tracing::info;

use crate::error::{Error, Result};
use std::io::Write;

/// Secrets are managed on the keyring and secrets are persisted
pub struct Secrets {}
impl Secrets {
    /// Set keyring secrets
    pub fn set(key: &str, secret: &str) -> Result<()> {
        info!("Set keyring secret");
        keyring::Entry::new(env!("CARGO_PKG_NAME"), key)?.set_secret(secret.trim().as_bytes())?;
        Ok(())
    }

    /// Get keyring secrets
    pub fn get(key: &str) -> Result<String> {
        let secret = keyring::Entry::new(env!("CARGO_PKG_NAME"), key)?.get_secret()?;
        String::from_utf8(secret).map_err(|e| Error::CustomError(e.to_string()))
    }

    /// Delete keyring secrets
    pub fn delete(key: &str) -> Result<()> {
        info!("Delete keyring secret");
        keyring::Entry::new(env!("CARGO_PKG_NAME"), key)?.delete_credential()?;
        Ok(())
    }
}

/// Used to read user inputs from the terminal
pub fn read_stdin() -> Result<String> {
    let mut string = String::new();
    std::io::stdin()
        .read_line(&mut string)
        .map_err(|_| Error::CustomError("Failed to read user input".to_string()))?;
    Ok(string.trim().to_string())
}

/// Used when requests inputs from user
/// print!() strings without line breaks will not flush, this method will force the flush()
pub fn promt_user(prompt: &str) -> Result<()> {
    // show the message to user
    println!("{prompt}");
    std::io::stdout().flush()?;

    Ok(())
}
