use crate::{storage::Storage, todo::Todo};
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn run(
    storage: &Storage,
    title: String,
    details: Option<String>,
    dua: Option<String>,
    priority: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let mut todos = storage.load()?;
    let mut todo = Todo::new(title);
    todo.details = details;
    if let Some(d) = dua {
        if let Ok(dt) = DateTime::parse_from_rfc3339(&d) {
            todo.due = Some(dt.with_timezone(&Utc));
        }
    }
    if let Some(p) = priority {
        todo.priority = match p.to_lowercase().as_str() {
            "low" => crate::todo::Priority::Low,
            "high" => crate::todo::Priority::High,
            _ => crate::todo::Priority::Medium,
        }
    }
    todo.tags = tags;
    todos.push(todo);
    storage.save(&todos)?;
    println!("Added task.");
    Ok(())
}
