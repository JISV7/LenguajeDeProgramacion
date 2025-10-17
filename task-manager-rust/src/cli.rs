use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Un gestor de tareas simple por CLI.",long_about = "Una herramienta de linea de comandos para gestionar pendientes.")]
#[command(disable_help_flag = false)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Añadir nueva tarea
    #[command(alias = "new")]
    Add {
        #[arg(short, long, help = "Nombre de la tarea")]
        name: String,
        #[arg(short, long, default_value = "", help = "Descripción de la tarea")]
        desc: String,
        #[arg(long, help = "Fecha de vencimiento (YYYY-MM-DD)")]
        due: String,
        #[arg(short, long, default_value = "", help = "Categorías separadas por comas")]
        cat: String,
    },
    /// Listar todas las tareas
    #[command(alias = "ls")]
    List {
        #[arg(short, long, help = "Filtrar tareas por categoría")]
        cat: Option<String>,
    },
    /// Actualizar una tarea existente
    #[command(alias = "mod")]
    Update {
        #[arg(short, long, help = "ID de la tarea a actualizar")]
        id: u32,
        #[arg(short, long, help = "Nuevo nombre de la tarea")]
        name: Option<String>,
        #[arg(short, long, help = "Nueva descripción de la tarea")]
        desc: Option<String>,
        #[arg(long, help = "Nueva fecha de vencimiento (YYYY-MM-DD)")]
        due: Option<String>,
        #[arg(short, long, help = "Nuevo estado (NoIniciado, EnProgreso, Completado)")]
        status: Option<String>,
        #[arg(short, long, help = "Nuevas categorías separadas por comas")]
        cat: Option<String>,
    },
    /// Eliminar una tarea
    #[command(alias = "rm")]
    Delete {
        #[arg(short, long, help = "ID de la tarea a eliminar")]
        id: u32,
    },
    /// Borra la pantalla de la terminal
    #[command(alias = "cls")]
    Clear,
}