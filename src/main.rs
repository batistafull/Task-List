use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    description: String,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\nThings to do:");
        println!("1. View tasks");
        println!("2. Add task");
        println!("3. Delete task");
        println!("4. Closer");

        print!("Select an option: ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Error reading entry");

        match option.trim() {
            "1" => {
                if tasks.is_empty() {
                    println!("There are no tasks.");
                } else {
                    println!("Tasks: ");
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}. {}", index + 1, task.description);
                    }
                }
            }
            "2" => {
                print!("Describe the new task: ");
                io::stdout().flush().expect("Error flushing stdout");

                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Error reading entry");
                let task = Task { description: description.trim().to_string() };
                tasks.push(task);
                println!("Added task.");
            }
            "3" => {
                if tasks.is_empty() {
                    println!("There are no tasks to delete.");
                } else {
                    print!("Enter the number of the task to delete: ");
                    io::stdout().flush().expect("Error flushing stdout");

                    let mut index = String::new();
                    io::stdin().read_line(&mut index).expect("Error reading entry");

                    match index.trim().parse::<usize>() {
                        Ok(i) if i > 0 && i <= tasks.len() => {
                            tasks.remove(i - 1);
                            println!("Task deleted.");
                        }
                        _ => println!("Invalid task number."),
                    }
                }
            }
            "4" => {
                println!("Coming out...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }

    println!("Press Enter to exit...");
    let mut salida = String::new();
    io::stdin().read_line(&mut salida).expect("Error reading entry");
}
