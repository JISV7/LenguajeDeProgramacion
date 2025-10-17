use crate::error::AppResult;
use crate::task::TaskManager;

/// Displays the tasks.
pub fn execute(manager: &TaskManager, cat: Option<String>) -> AppResult<bool> {
    manager.list_tasks(cat.as_deref());
    Ok(false) // Listing does not modify data.
}