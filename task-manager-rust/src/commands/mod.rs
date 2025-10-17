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

/// Manually prints the help text for the REPL context.
fn print_help() {
    println!("Gestor de Tareas CLI - Comandos disponibles:");
    println!("  add, new            Añadir nueva tarea");
    println!("    -n, --name <NAME>   Nombre de la tarea (Requerido)");
    println!("    -d, --desc <DESC>   Descripción de la tarea");
    println!("        --due <DUE>     Fecha de vencimiento (YYYY-MM-DD) (Requerido)");
    println!("    -c, --cat <CAT>     Categorías separadas por comas");
    println!();
    println!("  list, ls              Listar todas las tareas");
    println!("    -c, --cat <CAT>     Filtrar tareas por categoría");
    println!();
    println!("  update, mod           Actualizar una tarea");
    println!("    -i, --id <ID>       ID de la tarea a actualizar (Requerido)");
    println!("    -n, --name <NAME>   Nuevo nombre de la tarea");
    println!("    -d, --desc <DESC>   Nueva descripción de la tarea");
    println!("        --due <DUE>     Nueva fecha de vencimiento (YYYY-MM-DD)");
    println!("    -s, --status <STAT> Nuevo estado (NoIniciado, EnProgreso, Completado)");
    println!("    -c, --cat <CAT>     Nuevas categorías separadas por comas");
    println!();
    println!("  delete, rm            Eliminar una tarea");
    println!("    -i, --id <ID>       ID de la tarea a eliminar (Requerido)");
    println!();
    println!("  clear, cls            Borra la pantalla de la terminal");
    println!("  info                 Muestra este mensaje de ayuda");
    println!("  salir                 Salir de la aplicación");
}