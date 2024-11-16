---

# Task Prioritizer - A Task Management CLI App

This is a command-line application written in Rust that helps you manage your tasks with different priority levels. You can add tasks, view them, mark them as completed, change their priority, and save/load tasks to/from a file.

## Features
- **Add Task**: Add a task with a description and a priority level (Low, Medium, or High).
- **View Tasks**: View all tasks in a formatted table.
- **Mark Task as Completed**: Mark a task as completed.
- **Change Task Priority**: Change the priority of an existing task.
- **Automatic Save/Load**: The app will automatically load tasks from a file when started and save them when it exits.

## Menu

The app displays the following menu options:

```
+----------------------------+
|        Task Prioritizer     |
+----------------------------+
| 1. Add Task                |
| 2. View Tasks              |
| 3. Mark Task as Completed  |
| 4. Change Task Priority    |
| 0. Exit                    |
+----------------------------+
```

### Option Descriptions:
- **1. Add Task**: Prompts you to enter a task description and a priority (Low, Medium, or High). The task is added to the list.
- **2. View Tasks**: Displays all tasks with their priority and completion status in a table format.
- **3. Mark Task as Completed**: Allows you to mark a specific task as completed by its index.
- **4. Change Task Priority**: Lets you change the priority of a specific task.
- **0. Exit**: Exits the application, automatically saving the tasks to a file.

## Requirements

- Rust programming language (v1.50 or higher)
- Cargo (Rust’s package manager) to build and run the application.

## Installation

1. Clone the repository to your local machine:
   ```
   git clone https://github.com/t9fiction/Rust-task-prioritizer.git
   ```

2. Navigate to the project directory:
   ```
   cd task-prioritizer
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the application:
   ```
   cargo run
   ```

## How It Works

### Task Structure:
Each task has the following properties:
- **Description**: A string that represents the task.
- **Priority**: A task can have one of three priorities: Low, Medium, or High.
- **Completed**: A boolean flag indicating whether the task has been completed.

### File Handling:
- The tasks are stored in a file named `tasks.txt` in the same directory as the executable.
- The tasks are automatically loaded from this file when the app starts and saved when the app exits.

### Data Persistence:
- When the app starts, tasks from `tasks.txt` are loaded into memory.
- When you exit the application, all tasks are automatically saved back into the file.

## Usage

After starting the app, you can interact with it through the command-line interface:

1. **Add a task**: Select option `1` and provide the task description and priority.
2. **View tasks**: Select option `2` to view all tasks in a neatly formatted table.
3. **Mark a task as completed**: Select option `3` and provide the task index to mark it as completed.
4. **Change the priority of a task**: Select option `4` to change the priority of an existing task.
5. **Exit**: Select option `0` to save your tasks and exit the app.

## Example Output

When you choose the "View Tasks" option, tasks are displayed like this:

```
+---------------------------------+---------+-----------+------------+
|          Task Description       | Priority | Completed | 
+---------------------------------+---------+-----------+------------+
| Complete Rust tutorial          | High    | No        | 
| Buy groceries                    | Medium  | Yes       |
| Call the plumber                 | Low     | No        |
+---------------------------------+---------+-----------+------------+
```

## Contribution

If you would like to contribute to this project, feel free to fork the repository and submit a pull request. We welcome improvements to functionality, performance, and documentation.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
