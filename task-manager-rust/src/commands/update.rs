use crate::error::{AppError, AppResult};
use crate::task::{Status, TaskManager};
use chrono::NaiveDate;
use std::str::FromStr;

/// Processes the update of an existing task.
pub fn execute(
    manager: &mut TaskManager,
    id: u32,
    name: Option<String>,
    desc: Option<String>,
    due: Option<String>,
    status: Option<String>,
    cat: Option<String>,
) -> AppResult<bool> {
    if name.is_none() && desc.is_none() && due.is_none() && status.is_none() && cat.is_none() {
        return Err(AppError::Custom(
            "Se debe proporcionar al menos un flag (--name, --desc, --due, --status o --cat) para actualizar.".to_string(),
        ));
    }

    let task = manager.find_task_mut(id).ok_or(AppError::TaskNotFound(id))?;

    if let Some(n) = name {
        task.name = n;
    }
    if let Some(d) = desc {
        task.description = d;
    }
    if let Some(due_str) = due {
        let due_date = NaiveDate::parse_from_str(&due_str, "%Y-%m-%d")?;
        task.due_date = due_date.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(chrono::Utc).unwrap();
    }
    if let Some(s) = status {
        task.status = Status::from_str(&s)?;
    }
    if let Some(c) = cat {
        task.categories = c.split(',').map(str::trim).map(str::to_string).collect();
    }

    println!("Tarea con ID {} actualizada.", id);
    Ok(true) // Data has changed, needs to be saved.
}