use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub category: String,
    pub due_time: Option<NaiveDateTime>,
    pub status: bool,
}

impl Task {
    pub fn new(name: &str, description: &str, category: &str,  due_time: Option<NaiveDateTime>) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            due_time,
            status: false,
        }
    }
}

pub fn load_tasks() -> Vec<Task> {
    let data = fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())

}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write("tasks.json",data).expect("Unable to write file");
}