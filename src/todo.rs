use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub details: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub due: Option<DateTime<Utc>>,
    pub priority: Priority,
    pub tags: Vec<String>,
}
impl Todo {
    pub fn new<S: Into<String>>(title: S) -> Self {
        Todo {
            id: Uuid::new_v4(),
            title: title.into(),
            details: None,
            completed: false,
            created_at: Utc::now(),
            due: None,
            priority: Priority::Medium,
            tags: Vec::new(),
        }
    }
}
