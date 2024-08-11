use crate::task::Task;
use crate::commands::process_command::parse_datetime;

pub fn filter_tasks(tasks: &[Task], query: &str) {
    if query.is_empty() {
        for task in tasks {
            println!("{:?}", task);
        }
        return;
    }

    let predicates: Vec<&str> = query.split(" and ").collect();
    let filtered_tasks: Vec<&Task> = tasks.iter().filter(|task| {
        predicates.iter().all(|predicate| {
            apply_predicate(task, predicate)
        })
    }).collect();

    for task in filtered_tasks {
        println!("{:?}", task);
    }
}

fn apply_predicate(task: &Task, predicate: &str) -> bool {
    let predicate = predicate.trim();

    if predicate.starts_with("category=") {
        let value = predicate.trim_start_matches("category=").trim_matches('"');
        return task.category == value;
    }

    if predicate.starts_with("status=") {
        let value = predicate.trim_start_matches("status=").trim_matches('"');
        let status = value == "on";
        return task.status == status;
    }

    if predicate.starts_with("description like ") {
        let value = predicate.trim_start_matches("description like ").trim_matches('"');
        return task.description.contains(value);
    }

    if predicate.starts_with("name like ") {
        let value = predicate.trim_start_matches("name like ").trim_matches('"');
        return task.name.contains(value);
    }

    if predicate.starts_with("due_time <=") {
        return compare_due_time(task, predicate, "<=");
    }
    
    if predicate.starts_with("due_time <") {
        return compare_due_time(task, predicate, "<");
    }
    
    if predicate.starts_with("due_time >=") {
        return compare_due_time(task, predicate, ">=");
    }
    
    if predicate.starts_with("due_time >") {
        return compare_due_time(task, predicate, ">");
    }
    
    if predicate.starts_with("due_time =") {
        return compare_due_time(task, predicate, "=");
    }
    
    false
    }
    
    fn compare_due_time(task: &Task, predicate: &str, operator: &str) -> bool {
        let due_time_opt = task.due_time;
        if let Some(due_time) = due_time_opt {
            let value = predicate.trim_start_matches(&format!("due_time {}", operator)).trim();
            let value = value.trim_matches('"');
            let due_time_pred = match parse_datetime(value) {
                Ok(dt) => dt,
                Err(_) => {
                    println!("Ошибка парсинга даты из предиката: {}", value);
                    return false;
                }
            };
    
            match operator {
                "<" => return due_time < due_time_pred,
                "<=" => return due_time <= due_time_pred,
                ">" => return due_time > due_time_pred,
                ">=" => return due_time >= due_time_pred,
                "=" => return due_time == due_time_pred,
                _ => return false,
            }
        }
        false
    }