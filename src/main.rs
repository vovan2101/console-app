mod task;
mod commands;
mod filter;
mod ui;

use commands::process_command::process_command;
use task::{load_tasks, save_tasks};
use ui::{clear_terminal, print_welcome_message, print_tasks}; 

fn main() {
    clear_terminal();
    print_welcome_message();

    let mut tasks = load_tasks();
    print_tasks(&tasks);

    loop {

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if let Err(e) = process_command(input.trim(), &mut tasks) {
            eprintln!("Error: {}", e);
        }

        save_tasks(&tasks);
    }
}
