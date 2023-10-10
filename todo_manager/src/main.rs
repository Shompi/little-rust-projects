use std::io;

#[derive(PartialEq)]
enum TodoStatus {
    Complete,
    Incomplete,
}

#[derive(Debug)]
enum CustomError {
    ParseError,
    TaskNotFound,
    TaskNotCreated,
}
struct Task {
    id: usize,
    title: String,
    description: String,
    status: TodoStatus,
}

impl Task {
    fn new() -> Task {
        Task {
            id: 0,
            title: String::new(),
            description: String::new(),
            status: TodoStatus::Incomplete,
        }
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    // Initialize a TodoList
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn remove_task(&mut self, id: usize) {
        todo!("This operation has not yet been implemented.");
    }

    pub fn add_task(&mut self, mut task: Task) {
        task.id = self.tasks.len() + 1;
        self.tasks.push(task);
    }

    pub fn mark_as_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.get_mut(id - 1) {
            task.status = TodoStatus::Complete;
            println!("The task {} has been updated to Completed.\n", task.title)
        } else {
        }
    }

    pub fn list_tasks(&mut self) {
        for ele in &self.tasks {
            println!("id: {}\n", ele.id);
            println!("Title: {}\n", ele.title);
            println!("Description: {}\n", ele.description);

            if ele.status == TodoStatus::Complete {
                println!("Status: Completado\n");
            } else {
                println!("Status: No completado.\n");
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        print_menu();
        println!("Your choice: ");
        let mut choice = String::new();

        match io::stdin().read_line(&mut choice) {
            Ok(_) => {
                let number_choice: Result<i32, _> = choice.trim().parse();

                match number_choice {
                    Ok(number) => match number {
                        1 => {
                            // This is Add new Task
                            let task = create_task();

                            match task {
                                Ok(new_task) => todo_list.add_task(new_task),
                                Err(err) => println!("{:?}", err),
                            }
                        }
                        2 => {}
                        3 => todo_list.list_tasks(),
                        4 => {
                            let mut id_input = String::new();

                            io::stdin()
                                .read_line(&mut id_input)
                                .expect("Error parsing input");

                            let id: usize = id_input.parse().unwrap_or(0);

                            if id == 0 {
                                println!("The ID you just entered is invalid.\n");
                                continue;
                            }

                            todo_list.mark_as_done(id);
                        }
                        9 => break,
                        _ => println!("The choice you have entered is not valid."),
                    },
                    Err(_) => println!("There was an error parsing this integer."),
                }
            }
            Err(_) => println!("There was an error parsing user input."),
        }
    }
}

fn create_task() -> Result<Task, CustomError> {
    let mut title = String::new();
    let mut description = String::new();
    let mut new_task = Task::new();

    println!("Please enter the task title: ");
    match io::stdin().read_line(&mut title) {
        Ok(_) => {
            println!("Please enter this task's description: ");
            match io::stdin().read_line(&mut description) {
                Ok(_) => {
                    new_task.title = title;
                    new_task.description = description;
                    new_task.status = TodoStatus::Incomplete;
                    new_task.id = 0;

                    return Ok(new_task);
                }
                Err(_) => {
                    println!("There was an error parsing this input.");
                    return Err(CustomError::ParseError);
                }
            }
        }
        Err(_) => {
            println!("There was an error parsing this input.");
            return Err(CustomError::ParseError);
        }
    }
}

fn print_menu() {
    println!("Welcome to this TODO manager!\n");
    println!("Select your operation:\n");
    println!("1) Add a new TODO\n");
    println!("2) Remove a TODO\n");
    println!("3) List all TODOs\n");
    println!("4) Mark TODO as done\n");
    println!("\n");
}
