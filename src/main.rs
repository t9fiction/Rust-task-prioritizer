use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Clone)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
struct Task {
    description: String,
    priority: Priority,
    completed: bool,
}

impl Task {
    fn new(description: String, priority: Priority) -> Self {
        Self {
            description,
            priority,
            completed: false,
        }
    }
}

/// Helper function to get user input
fn get_user_choice(prompt: &str) -> i8 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush output");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i8>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input, please enter a valid number."),
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().expect("Failed to flush output");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn add_task(tasks: &mut VecDeque<Task>) {
    let description = get_user_input("Enter task description: ");
    let priority = match get_user_choice("Enter task priority (1: Low, 2: Medium, 3: High): ") {
        1 => Priority::Low,
        2 => Priority::Medium,
        3 => Priority::High,
        _ => {
            println!("Invalid priority, defaulting to Medium.");
            Priority::Medium
        }
    };
    let task = Task::new(description, priority);
    tasks.push_back(task);
    println!("\nTask added successfully!");
    print_task_table(&tasks);
}

fn view_tasks(tasks: &VecDeque<Task>) {
    if tasks.is_empty() {
        println!("\nNo tasks found.");
        return;
    }

    println!(
        "\n{:<5} {:<40} {:<10} {:<10}",
        "Index", "Description", "Priority", "Status"
    );
    println!("{:-<5} {:-<40} {:-<10} {:-<10}", "", "", "", "");

    let mut sorted_tasks = tasks.clone();
    sorted_tasks
        .make_contiguous()
        .sort_by_key(|task| match task.priority {
            Priority::High => 1,
            Priority::Medium => 2,
            Priority::Low => 3,
        });

    for (index, task) in sorted_tasks.iter().enumerate() {
        let status = if task.completed {
            "Completed"
        } else {
            "Pending"
        };
        let priority = match task.priority {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        println!(
            "{:<5} {:<40} {:<10} {:<10}",
            index + 1,
            task.description,
            priority,
            status
        );
    }
}

fn mark_complete(tasks: &mut VecDeque<Task>) {
    let index = get_user_choice("Enter the index of the task to mark as completed: ") as usize;
    let index = index - 1;
    if index > tasks.len() {
        println!("Invalid task index.");
        return;
    }

    if tasks[index].completed {
        println!("Task is already marked as completed.");
        return;
    }
    tasks[index].completed = true;
    println!("\nTask marked as completed successfully!");
    print_task_table(&tasks);
}

fn change_priority(tasks: &mut VecDeque<Task>) {
    let index = get_user_choice("Enter the index of the task to change priority: ") as usize;
    let index = index - 1;
    if index > tasks.len() {
        println!("Invalid task index.");
        return;
    }

    let new_priority = match get_user_choice("Enter new priority (1: Low, 2: Medium, 3: High): ") {
        1 => Priority::Low,
        2 => Priority::Medium,
        3 => Priority::High,
        _ => {
            println!("Invalid priority, defaulting to Medium.");
            Priority::Medium
        }
    };

    tasks[index].priority = new_priority;
    println!("\nTask priority updated successfully!");
    print_task_table(&tasks);
}

fn save_tasks(tasks: &VecDeque<Task>) {
    let mut file = File::create("tasks.txt").expect("Failed to create file");
    for task in tasks {
        writeln!(
            file,
            "{},{},{}",
            task.description,
            match task.priority {
                Priority::Low => "Low",
                Priority::Medium => "Medium",
                Priority::High => "High",
            },
            task.completed
        )
        .expect("Failed to write to file");
    }
}

fn load_tasks_from_file(tasks: &mut VecDeque<Task>) {
    let file = File::open("tasks.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);

    tasks.clear();
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 3 {
                let description = parts[0].to_string();
                let priority = match parts[1].trim() {
                    "Low" => Priority::Low,
                    "Medium" => Priority::Medium,
                    "High" => Priority::High,
                    _ => continue,
                };
                let completed = parts[2].trim() == "true";

                tasks.push_back(Task {
                    description,
                    priority,
                    completed,
                });
            }
        }
    }
    println!("\nTasks loaded successfully!");
}

fn print_task_table(tasks: &VecDeque<Task>) {
    println!(
        "\n{:<5} {:<40} {:<10} {:<10}",
        "Index", "Description", "Priority", "Status"
    );
    println!("{:-<5} {:-<40} {:-<10} {:-<10}", "", "", "", "");
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed {
            "Completed"
        } else {
            "Pending"
        };
        let priority = match task.priority {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        };
        println!(
            "{:<5} {:<40} {:<10} {:<10}",
            i + 1,
            task.description,
            priority,
            status
        );
    }
}

fn display_menu() {
    println!("\n+----------------------------+");
    println!("|        Task Prioritizer     |");
    println!("+----------------------------+");
    println!("| 1. Add Task                |");
    println!("| 2. View Tasks              |");
    println!("| 3. Mark Task as Completed  |");
    println!("| 4. Change Task Priority    |");
    println!("| 0. Exit                    |");
    println!("+----------------------------+");
}

fn main() {
    let mut tasks: VecDeque<Task> = VecDeque::new();

    // Load tasks from file automatically at the start
    load_tasks_from_file(&mut tasks);

    loop {
        display_menu();

        let choice = get_user_choice("Enter your choice: ");

        match choice {
            0 => {
                // Save tasks to file automatically when exiting
                save_tasks(&tasks);
                println!("Tasks saved successfully. Exiting...");
                break;
            }
            1 => {
                add_task(&mut tasks);
            }
            2 => {
                view_tasks(&tasks);
            }
            3 => {
                mark_complete(&mut tasks);
            }
            4 => {
                change_priority(&mut tasks);
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
