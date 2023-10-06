# Disclaimer

> This assignment was prompted by CHATGPT 3.5

## Assignment: Build a To-Do List Manager

> Create a console-based to-do list manager. The program should allow users to add tasks, mark tasks as done, list all tasks, and remove tasks from the to-do list. Here are the steps to guide you through the assignment:

### Data Structure

Define a struct `Task` that represents a to-do item. Each task should have a unique identifier (you can use a counter for this), a description, and a status (done or not done).

### Operations

Implement functions/methods for adding a task, marking a task as done, listing all tasks, and removing a task. You can use a `Vec<Task>` to store the tasks.

### User Interface

* Create a **loop** where the user can interact with the program. Provide options for **adding a task**, **marking a task as done**, **listing all tasks**, and **removing a task**.
* Print a menu for the user to choose from and read the user's choice.

### Example Interaction

* User chooses to add a task: Add a new task: Buy groceries
* User chooses to list tasks: 1. Buy groceries (Not Done)
* User chooses to mark a task as done: Enter the task number to mark as done: 1
* User chooses to list tasks again: 1. Buy groceries (Done)
* User chooses to remove a task: Enter the task number to remove: 1
* User chooses to list tasks: No tasks remaining.
  
## Additional Challenges

* Implement task persistence: Save tasks to a file, so they are not lost when the program exits and reload them when the program starts.
Add due dates to tasks and allow sorting tasks by due date.

* Implement different lists (e.g., work tasks, personal tasks) and allow users to switch between lists.
* Create a user-friendly date input system, allowing users to input due dates in a natural language format (e.g., "tomorrow," "next Friday").
* Implement a priority system for tasks and allow sorting tasks by priority.

This assignment will help you practice creating and managing data structures, user input/output, loops, and conditional logic in Rust. Have fun coding!
