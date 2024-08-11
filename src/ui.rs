use crate::task::Task;
use std::fmt;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        std::process::Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}

pub fn print_welcome_message() {
    println!(r#"
    ==========================================
    |                                         |
    |          WELCOME TO TODO LIST!          |
    |                                         |
    ==========================================
    "#);
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "| {:^10} | {:^30} | {:^15} | {:^20} | {:^10} |",
            self.name,
            self.description,
            self.category,
            self.due_time.map_or("N/A".to_string(), |dt| dt.format("%Y-%m-%d %H:%M").to_string()),
            if self.status { "Done" } else { "Pending" }
        )
    }
}

pub fn print_tasks(tasks: &[Task]) {
    println!("+------------+--------------------------------+-----------------+----------------------+------------+");
    println!("| Task Name  | Description                    | Category        | Due Time             | Status     |");
    println!("+------------+--------------------------------+-----------------+----------------------+------------+");

    for task in tasks {
        println!("{}", task);
        println!("+------------+--------------------------------+-----------------+----------------------+------------+");
    }
}
