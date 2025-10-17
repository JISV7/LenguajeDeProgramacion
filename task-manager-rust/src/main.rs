mod cli;
mod commands;
mod error;
mod storage;
mod task;

use crate::cli::Cli;
use crate::commands::handle_command;
use crate::error::AppResult;
use crate::storage::{load_tasks, save_tasks};
use crate::task::TaskManager;
use clap::Parser;
use std::io::{self, Write};

fn run_app() -> AppResult<()> {
    // Load existing tasks from the JSON file on startup.
    let tasks = load_tasks()?;
    let mut task_manager = TaskManager::new(tasks);

    println!("Bienvenido al Gestor de Tareas CLI");
    println!("Escribe 'help' para ver los comandos, o 'exit' para terminar.");

    // Start the interactive loop (REPL).
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye Sayonara Adiós");
            break;
        }
        if input.is_empty() {
            continue;
        }

        // shlex splits the input respecting quotes.
        let args = match shlex::split(input) {
            Some(args) => args,
            None => {
                eprintln!("Error: Entrada de comando inválida, verifique las comillas.");
                continue;
            }
        };

        // clap requires the first argument to be the binary name, placeholder.
        let mut clap_args = vec!["task-manager-rust"];
        clap_args.extend(args.iter().map(String::as_str));

        match Cli::try_parse_from(&clap_args) {
            Ok(cli) => {
                // handle_command returns Ok(true) if data changed and needs to be saved.
                match handle_command(&mut task_manager, cli.command) {
                    Ok(needs_saving) if needs_saving => {
                        if let Err(e) = save_tasks(&task_manager.tasks) {
                            eprintln!("Error al guardar las tareas: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                    _ => {} // Ok(false) does nothing.
                }
            }
            // clap handles and prints its own parsing errors (command not found, wrong flags, etc.).
            Err(e) => e.print().unwrap_or(()),
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Error de aplicación: {}", e);
        std::process::exit(1);
    }
}