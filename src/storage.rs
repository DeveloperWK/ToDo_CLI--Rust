use crate::todo::Todo;
use anyhow::Result;
use directories::ProjectDirs;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let proj = ProjectDirs::from("dev", "wasiful_kabir", "todo-cli")
            .expect("cannot determine project dir");
        let data_dir = proj.data_dir();
        fs::create_dir(data_dir).ok();
        let path = data_dir.join("todos.json");
        Storage { path }
    }
    pub fn load(&self) -> Result<Vec<Todo>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let s = fs::read_to_string(&self.path)?;
        let todos = serde_json::from_str(&s)?;
        Ok(todos)
    }
    pub fn save(&self, todos: &[Todo]) -> Result<()> {
        let tmp = self.path.with_extension("tmp");
        let s = serde_json::to_string_pretty(todos)?;
        fs::write(&tmp, s)?;
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }
}
