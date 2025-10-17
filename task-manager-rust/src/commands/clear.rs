use crate::error::{AppError, AppResult};
use std::process::Command;

/// Executes the OS-specific command to clear the screen.
pub fn execute() -> AppResult<bool> {
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("clear")
    };

    if cfg!(target_os = "windows") {
        cmd.args(["/c", "cls"]);
    }

    // status() executes the command and waits for it to finish.
    let status = cmd.status()?;

    if status.success() {
        Ok(false) // Clearing the screen does not modify data.
    } else {
        Err(AppError::Custom(
            "No se pudo limpiar la pantalla.".to_string(),
        ))
    }
}