use crate::error::AppResult;
use crate::task::TaskManager;
use chrono::NaiveDate;

/// Processes the addition of a new task.
pub fn execute(
    manager: &mut TaskManager,
    name: String,
    desc: String,
    due: String,
    cat: String,
) -> AppResult<bool> {
    let due_date = NaiveDate::parse_from_str(&due, "%Y-%m-%d")?;
    let categories = split_categories(&cat);
    manager.add_task(name, desc, due_date, categories);
    Ok(true) // Data has changed, needs to be saved.
}

/// Helper function to split the comma-separated category string.
fn split_categories(cat_str: &str) -> Vec<String> {
    if cat_str.is_empty() {
        vec![]
    } else {
        cat_str.split(',').map(str::trim).map(str::to_string).collect()
    }
}