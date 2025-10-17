use crate::error::{AppError, AppResult};
use chrono::{DateTime, NaiveDate, Utc};
use comfy_table::{presets::UTF8_FULL, Cell, ContentArrangement, Table};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Enum representing the status of a task.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

/// Implements the display for "Status".
impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::NotStarted => write!(f, "Not started"),
            Status::InProgress => write!(f, "In progress"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

/// Allows converting a string slice (&str) into a "Status".
/// Accepts various forms for user convenience (e.g., "c", "completado").
impl FromStr for Status {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace('_', " ").as_str() {
            "no iniciado" | "n" => Ok(Status::NotStarted),
            "en progreso" | "i" => Ok(Status::InProgress),
            "completado" | "c" => Ok(Status::Completed),
            _ => Err(AppError::InvalidStatus(s.to_string())),
        }
    }
}

/// Represents a single to-do item with all its attributes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub creation_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub status: Status,
    pub categories: Vec<String>,
}

/// Manages the collection of tasks and related business logic.
pub struct TaskManager {
    pub tasks: Vec<Task>,
    last_id: u32,
}

impl TaskManager {
    /// Creates a new "TaskManager" from a list of loaded tasks.
    pub fn new(tasks: Vec<Task>) -> Self {
        let last_id = tasks.iter().map(|t| t.id).max().unwrap_or(0);
        TaskManager { tasks, last_id }
    }

    /// Gets the next unique ID for a new task.
    fn next_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    /// Adds a new task to the list.
    pub fn add_task(
        &mut self,
        name: String,
        description: String,
        due_date: NaiveDate,
        categories: Vec<String>,
    ) {
        let id = self.next_id();
        let new_task = Task {
            id,
            name: name.clone(),
            description,
            creation_date: Utc::now(),
            due_date: due_date.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap(),
            status: Status::NotStarted,
            categories,
        };
        self.tasks.push(new_task);
        println!("Tarea '{}' (ID: {}) añadida.", name, id);
    }

    /// Finds a mutable reference to a task by its ID.
    pub fn find_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    /// Deletes a task by its ID.
    pub fn delete_task(&mut self, id: u32) -> AppResult<()> {
        let initial_len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        if self.tasks.len() == initial_len {
            Err(AppError::TaskNotFound(id))
        } else {
            println!("Tarea con ID {} eliminada.", id);
            Ok(())
        }
    }

    /// Displays tasks in a formatted table.
    /// Allows filtering by category and sorts tasks by status and due date.
    pub fn list_tasks(&self, category_filter: Option<&str>) {
        let mut tasks_to_list: Vec<_> = self.tasks.iter().collect();

        if let Some(cat) = category_filter {
            tasks_to_list.retain(|t| t.categories.iter().any(|c| c.eq_ignore_ascii_case(cat)));
        }

        if tasks_to_list.is_empty() {
            if let Some(cat) = category_filter {
                println!("No se encontraron tareas con la categoría: {}", cat);
            } else {
                println!("No hay tareas.");
            }
            return;
        }

        // Sort: completed tasks go to the bottom, the rest by due date.
        tasks_to_list.sort_by(|a, b| {
            match (a.status == Status::Completed, b.status == Status::Completed) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                _ => a.due_date.cmp(&b.due_date),
            }
        });

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec!["ID", "ESTADO", "VENCE", "NOMBRE", "CATEGORÍAS", "DESCRIPCIÓN"]);

        for task in tasks_to_list {
            let description = if task.description.len() > 40 {
                format!("{}...", &task.description[..37])
            } else {
                task.description.clone()
            };
            table.add_row(vec![
                Cell::new(task.id.to_string()),
                Cell::new(task.status.to_string()),
                Cell::new(task.due_date.format("%Y-%m-%d").to_string()),
                Cell::new(&task.name),
                Cell::new(task.categories.join(", ")),
                Cell::new(description),
            ]);
        }
        println!("{table}");
    }
}