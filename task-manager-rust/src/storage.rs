use crate::error::AppResult;
use crate::task::Task;
use std::fs;
use std::io::ErrorKind;

const DB_FILE: &str = "tasks.json";

/// Saves a slice of tasks to the `tasks.json` file.
pub fn save_tasks(tasks: &[Task]) -> AppResult<()> {
    // `serde_json::to_string_pretty` formats the JSON for readability.
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(DB_FILE, data)?;
    Ok(())
}

/// Loads tasks from the `tasks.json` file.
/// If the file does not exist, it returns an empty list.
pub fn load_tasks() -> AppResult<Vec<Task>> {
    match fs::read_to_string(DB_FILE) {
        Ok(data) => {
            if data.is_empty() {
                Ok(Vec::new())
            } else {
                let tasks: Vec<Task> = serde_json::from_str(&data)?;
                Ok(tasks)
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}