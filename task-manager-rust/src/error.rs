use thiserror::Error;

// Defines all possible errors that can occur within the application.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de serializar/deserializar JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error al insertar fecha: {0}")]
    Chrono(#[from] chrono::ParseError),

    #[error("No existe la tarea con ID {0}")]
    TaskNotFound(u32),

    #[error("Status invalido: '{0}'")]
    InvalidStatus(String),
    
    #[error("{0}")]
    Custom(String),
}

pub type AppResult<T> = Result<T, AppError>;