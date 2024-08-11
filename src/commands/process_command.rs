use crate::task::Task;
use crate::commands::parse_command::parse_command;
use crate::commands::task_operations::{add_task, mark_task_done, delete_task, update_task};
use crate::filter::filter_tasks;
use chrono::NaiveDateTime;

pub fn process_command(command: &str, tasks: &mut Vec<Task>) -> Result<(), String> {
    let parts: Vec<String> = parse_command(command)?;

    match parts[0].as_str() {
        "add" => {
            if parts.len() != 5 {
                return Err("Add name, description, category and due_time".to_string());
            }

            let name = &parts[1];
            let description = &parts[2];
            let category = &parts[3];
            let date = if parts.len() == 5 {
                Some(parse_datetime(&parts[4])?)
            } else {
                None
            };

            add_task(tasks, name, description, category, date);
            Ok(())
        }
        "done" => {
            if parts.len() != 2 {
                return Err("Usage: done <name>".to_string());
            }

            let name = &parts[1];
            mark_task_done(tasks, name);
            Ok(())
        }
        "delete" => {
            if parts.len() != 2 {
                return Err("Usage: delete <name>".to_string());
            }

            let name = &parts[1];
            delete_task(tasks, name);
            Ok(())
        }
        "update" => {
            if parts.len() != 2 {
                return Err("Usage: update <name>".to_string());
            }

            let name = &parts[1];
            update_task(tasks, name);
            Ok(())
        }
        "select" => {
            let query = command.strip_prefix("select * where").unwrap_or("");
            filter_tasks(tasks, query);
            Ok(())
        }
        _ => Err(format!("Unknown command: {}", parts[0])),
    }
}

pub fn parse_datetime(input: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M")
    .map_err(|_| "Invalid date format. Use: YYYY-MM-DD HH:MM".to_string())
}