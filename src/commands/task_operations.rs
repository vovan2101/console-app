use crate::task::{Task, save_tasks};
use chrono::NaiveDateTime;
use crate::commands::process_command::parse_datetime;

pub fn delete_task(tasks: &mut Vec<Task>, name: &str) {
    if let Some(pos) = tasks.iter().position(|task| task.name == name) {
        tasks.remove(pos);
        println!("Task '{}' deleted.", name);
    } else {
        println!("Task '{}' not found.", name);
    }
    save_tasks(tasks);
}

pub fn mark_task_done(tasks: &mut Vec<Task>, name: &str) {
    if let Some(task) = tasks.iter_mut().find(|task| task.name == name) {
        task.status = true;
        println!("Task '{}' marked as done.", name);
    } else {
        println!("Task '{}' not found.", name);
    }
    save_tasks(tasks);
}

pub fn add_task(tasks: &mut Vec<Task>, name: &str, description: &str, category: &str, due_time: Option<NaiveDateTime>) {
    let task = Task::new(name, description, category, due_time);
    tasks.push(task);
    save_tasks(&tasks);
}

pub fn update_task(tasks: &mut Vec<Task>, name: &str) {
    if let Some(task) = tasks.iter_mut().find(|task| task.name == name) {
        println!("Updating task '{}'", name);

        let new_name = prompt("Enter new name (leave empty to keep current): ");
        if !new_name.trim().is_empty() {
            task.name = new_name;
        }

        let new_description = prompt("Enter new description (leave empty to keep current): ");
        if !new_description.trim().is_empty() {
            task.description = new_description;
        }

        let new_category = prompt("Enter new category (leave empty to keep current): ");
        if !new_category.trim().is_empty() {
            task.category = new_category;
        }

        let new_due_date = prompt("Enter new due time (YYYY-MM-DD HH:MM, leave empty to keep current): ");
        if !new_due_date.trim().is_empty() {
            task.due_time = Some(parse_datetime(&new_due_date).expect("Invalid due_time format"));
        }

        println!("Task '{}' updated.", task.name);
    } else {
        println!("Task '{}' not found.", name);
    }

    save_tasks(tasks);
}

fn prompt(message: &str) -> String {
    use std::io::{self, Write};

    print!("{}", message);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}