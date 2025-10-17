use crate::error::AppResult;
use crate::task::TaskManager;

/// Processes the deletion of a task.
pub fn execute(manager: &mut TaskManager, id: u32) -> AppResult<bool> {
    manager.delete_task(id)?;
    Ok(true) // Data has changed, needs to be saved.
}