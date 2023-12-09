use prettytable::{color, Attr, Cell, Row, Table};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

#[derive(Debug, Clone)]
struct TodoItem {
    id: u32,
    done: bool,
    title: String,
}

fn generate_id(todos: &Vec<TodoItem>) -> u32 {
    (todos.len() + 1).try_into().unwrap()
}

fn paint_cell(text: String, color: String) {
    let length_text = text.chars().count();
    let dashes = "-".repeat(length_text + 2);
    println!("{}+{}+", color, dashes);
    println!("| {} |", text);
    println!("+{}{}+\x1b[0m", color, dashes);
}

// Load all todos from file
fn load_all_todos(filename: &str) -> io::Result<Vec<TodoItem>> {
    let mut todos: Vec<TodoItem> = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(todo) = parse_todo_line(&line) {
                todos.push(todo)
            }
        }
    }

    Ok(todos)
}

// Save a new TODO to file
fn write_new_todo(filename: &str, todos: &[TodoItem]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).open(filename)?;

    for todo in todos {
        let line = format!("{}__{}__{}\n", todo.id, todo.done, todo.title);
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}

fn parse_todo_line(line: &str) -> Option<TodoItem> {
    let parts: Vec<&str> = line.split("__").collect();

    if parts.len() == 3 {
        if let Ok(id) = parts[0].parse() {
            let done = match parts[1] {
                "true" => true,
                "false" => false,
                _ => return None,
            };
            let title = parts[2].to_string();

            return Some(TodoItem { id, done, title });
        }
    }

    None
}

// Function [main]
fn main() -> io::Result<()> {
    let mut todos: Vec<TodoItem> = load_all_todos("TODOS")?;

    loop {
        let mut input = String::new();
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("#")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("COMPLETED")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("TITLE")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
        ]));

        println!("What are you going to do?: \x1b[32m[a]\x1b[0mdd  \x1b[32m[l]\x1b[0mist  \x1b[32m[d]\x1b[0melete  \x1b[32m[e]\x1b[0mdit  \x1b[32m[q]\x1b[0muit");
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read user data");

        let input = input.trim();

        match input {
            "a" => {
                let mut title = String::new();

                println!("To create a new TODO you must add the following fields:");
                println!("title: [String]");

                print!("\x1b[32mEnter the title > \x1b[0m");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).expect("Dont read");

                println!();

                let new_todo = TodoItem {
                    id: generate_id(&todos),
                    done: false,
                    title,
                };

                todos.push(new_todo);
                write_new_todo("TODOS", &todos)?;

                for todo in todos.iter() {
                    table.add_row(Row::new(vec![
                        Cell::new(&todo.id.to_string()),
                        Cell::new(&todo.done.to_string()),
                        Cell::new(&todo.title),
                    ]));
                }

                table.printstd();
                println!();
            }
            "l" => {
                for todo in todos.iter() {
                    table.add_row(Row::new(vec![
                        Cell::new(&todo.id.to_string()),
                        Cell::new(&todo.done.to_string()),
                        Cell::new(&todo.title),
                    ]));
                }
                table.printstd();
                println!()
            }
            "d" => loop {
                let mut input = String::new();
                print!("Enter the ID of the TODO to delete it > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read what the user entered");

                let todo_id = input.trim();

                if todos.len() > 0 {
                    let found_todo = todos.iter().find(|&todo| {
                        let id_entered_by_user = todo_id
                            .parse()
                            .expect("There was an error converting the ID entered by the user");
                        todo.id == id_entered_by_user
                    });

                    match found_todo {
                        Some(todo) => {
                            if let Some(index) = todos
                                .iter()
                                .position(|current_todo| current_todo.id == todo.id)
                            {
                                let mut table_removed_todo = Table::new();
                                let removed_todo = todos.remove(index);

                                println!("\x1b[32mThe TODO has been successfully deleted\n\x1b[0m");
                                table_removed_todo.add_row(Row::new(vec![
                                    Cell::new(&removed_todo.id.to_string())
                                        .with_style(Attr::ForegroundColor(color::GREEN)),
                                    Cell::new(&removed_todo.done.to_string()),
                                    Cell::new(&removed_todo.title),
                                ]));
                                table_removed_todo.printstd();
                                println!("");
                            } else {
                                paint_cell(
                                    String::from("The TODO was not found in the list"),
                                    String::from("\x1b[33m"),
                                );
                            }
                        }
                        None => paint_cell(String::from("TODO does not exist"), String::from("")),
                    }
                    break;
                } else {
                    paint_cell(
                        String::from("No TODOS available, add a TODO first"),
                        String::from("\x1b[33m"),
                    );
                }
            },
            "e" => loop {
                let mut input = String::new();
                print!("Enter th first 3 characters of the TODO ID to edit > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read what the user entered");

                let todo_id = input.trim();

                if todos.len() > 0 {
                    let found_todo = todos.iter().find(|&todo| {
                        let id_entered_by_user = todo_id
                            .parse()
                            .expect("There was an error converting the ID entered by the user");
                        todo.id == id_entered_by_user
                    });

                    match found_todo {
                        Some(todo) => {
                            if let Some(index) = todos
                                .iter()
                                .position(|current_todo| current_todo.id == todo.id)
                            {
                                let mut edited_todo = todos[index].clone();

                                let mut title = String::new();

                                print!("\x1b[32mEnter the new title > \x1b[0m");
                                io::stdout().flush().unwrap();
                                io::stdin().read_line(&mut title).expect("Dont read");
                                edited_todo.title = title.trim().to_string();

                                println!();

                                todos[index] = edited_todo;

                                let mut table_edited_todo = Table::new();
                                println!("The TODO has been successfully edited\n");
                                table_edited_todo.add_row(Row::new(vec![
                                    Cell::new(&todos[index].id.to_string()),
                                    Cell::new(&todos[index].done.to_string()),
                                    Cell::new(&todos[index].title),
                                ]));
                                table_edited_todo.printstd();
                            } else {
                                paint_cell(
                                    String::from("The TODO was not found in the list"),
                                    String::from("\x1b[33m"),
                                );
                            }
                        }
                        None => paint_cell(String::from("TODO does not exist"), String::from("")),
                    }
                    break;
                } else {
                    paint_cell(
                        String::from("No TODOS available, add a TODO first"),
                        String::from("\x1b[33m"),
                    );
                }
            },
            "q" => break,
            _ => {
                let mut table_invalid_option = Table::new();
                table_invalid_option
                    .add_row(Row::new(vec![Cell::new("Not a valid option")
                        .with_style(Attr::ForegroundColor(color::RED))]));
                table_invalid_option.printstd();
                println!()
            }
        }
    }

    Ok(())
}
