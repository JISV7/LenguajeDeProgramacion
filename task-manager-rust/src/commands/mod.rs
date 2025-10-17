mod add;
mod clear;
mod delete;
mod list;
mod update;

use crate::cli::Commands;
use crate::error::AppResult;
use crate::task::TaskManager;

/// Dispatches the parsed command to the appropriate handler function.
/// Returns a boolean indicating if the task state was modified and needs to be saved.
pub fn handle_command(
    manager: &mut TaskManager,
    command: Commands,
) -> AppResult<bool> {
    match command {
        Commands::Add { name, desc, due, cat } => add::execute(manager, name, desc, due, cat),
        Commands::List { cat } => list::execute(manager, cat),
        Commands::Update { id, name, desc, due, status, cat } => {
            update::execute(manager, id, name, desc, due, status, cat)
        }
        Commands::Delete { id } => delete::execute(manager, id),
        Commands::Clear => clear::execute(),
    }
}