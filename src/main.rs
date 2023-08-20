use prettytable::{color, Attr, Cell, Row, Table};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct TodoItem {
    id: String,
    title: String,
    description: String,
    done: bool,
}

// ?: Generate random UUID
fn generate_uuid() -> String {
    let lenght = 24;
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect()
}

fn paint_cell(text: String, color: String) {
    let length_text = text.chars().count();
    let dashes = "-".repeat(length_text + 2);
    println!("{}+{}+", color, dashes);
    println!("| {} |", text);
    println!("+{}{}+\x1b[0m", color, dashes);
}

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();

    let todo_1 = TodoItem {
        id: generate_uuid(),
        title: String::from("Todo 01"),
        description: String::from("Description of todo 01"),
        done: false,
    };
    let todo_2 = TodoItem {
        id: generate_uuid(),
        title: String::from("Todo 02"),
        description: String::from("Description of todo 02"),
        done: false,
    };

    todos.push(todo_1);
    todos.push(todo_2);

    loop {
        let mut input = String::new();
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("#")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("TITLE")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("DESCRIPTION")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
            Cell::new("COMPLETED")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN)),
        ]));

        println!("What are you going to do?: \x1b[32m[a]\x1b[0m Add  \x1b[32m[l]\x1b[0m To list  \x1b[32m[d]\x1b[0m Delete  \x1b[32m[e]\x1b[0m Edit  \x1b[32m[q]\x1b[0m Quit");
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read user data");

        let input = input.trim();

        match input {
            "a" => {
                let mut title = String::new();
                let mut description = String::new();

                println!("To create a new TODO you must add the following fields:");
                println!("title: [String]");
                println!("description?: [String] [Optional]\n");

                print!("\x1b[32mEnter the title > \x1b[0m");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).expect("Dont read");

                print!("\x1b[32mEnter the description > \x1b[0m");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).expect("Dont");
                println!();

                let new_todo = TodoItem {
                    id: generate_uuid(),
                    title,
                    description,
                    done: false,
                };

                todos.push(new_todo);

                for todo in todos.iter() {
                    table.add_row(Row::new(vec![
                        Cell::new(&todo.id),
                        Cell::new(&todo.title),
                        Cell::new(&todo.description),
                        Cell::new(&todo.done.to_string()),
                    ]));
                }

                table.printstd();
                println!();
            }
            "l" => {
                for todo in todos.iter() {
                    table.add_row(Row::new(vec![
                        Cell::new(&todo.id),
                        Cell::new(&todo.title),
                        Cell::new(&todo.description),
                        Cell::new(&todo.done.to_string()),
                    ]));
                }
                table.printstd();
                println!();
            }
            "d" => loop {
                let mut input = String::new();
                print!("Enter th first 3 characters of the TODO ID to delete > ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Could not read what the user entered");

                let todo_id = input.trim();

                if todo_id.len() <= 3 && todo_id.len() > 2 {
                    let found_todo = todos.iter().find(|&todo| {
                        let first_tree_chars_of_id: String = todo.id.chars().take(3).collect();
                        first_tree_chars_of_id == todo_id
                    });

                    match found_todo {
                        Some(todo) => {
                            if let Some(index) = todos
                                .iter()
                                .position(|current_todo| current_todo.id == todo.id)
                            {
                                let mut table_removed_todo = Table::new();
                                let removed_todo = todos.remove(index);

                                println!("\x1b[32m");
                                println!("The TODO has been successfully deleted\n");
                                table_removed_todo.add_row(Row::new(vec![
                                    Cell::new(&removed_todo.id),
                                    Cell::new(&removed_todo.title),
                                    Cell::new(&removed_todo.description),
                                    Cell::new(&removed_todo.done.to_string()),
                                ]));
                                table_removed_todo.printstd();
                                println!("\x1b[0m");
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
                } else if todos.len() < 1 {
                    paint_cell(
                        String::from("No TODOS available, add a TODO first"),
                        String::from("\x1b[33m"),
                    );
                } else {
                    paint_cell(
                        String::from("Enter only 3 characters"),
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

                if todo_id.len() <= 3 && todo_id.len() > 2 {
                    let found_todo = todos.iter().find(|&todo| {
                        let first_tree_chars_of_id: String = todo.id.chars().take(3).collect();
                        first_tree_chars_of_id == todo_id
                    });

                    match found_todo {
                        Some(todo) => {
                            if let Some(index) = todos
                                .iter()
                                .position(|current_todo| current_todo.id == todo.id)
                            {
                                let mut edited_todo = todos[index].clone();

                                let mut title = String::new();
                                let mut description = String::new();

                                print!("\x1b[32mEnter the new title > \x1b[0m");
                                io::stdout().flush().unwrap();
                                io::stdin().read_line(&mut title).expect("Dont read");
                                edited_todo.title = title.trim().to_string();

                                print!("\x1b[32mEnter the new description > \x1b[0m");
                                io::stdout().flush().unwrap();
                                io::stdin().read_line(&mut description).expect("Dont");
                                edited_todo.description = description.trim().to_string();
                                println!();

                                todos[index] = edited_todo;

                                let mut table_edited_todo = Table::new();
                                println!("The TODO has been successfully edited\n");
                                table_edited_todo.add_row(Row::new(vec![
                                    Cell::new(&todos[index].id),
                                    Cell::new(&todos[index].title),
                                    Cell::new(&todos[index].description),
                                    Cell::new(&todos[index].done.to_string()),
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
                } else if todos.len() < 1 {
                    paint_cell(
                        String::from("No TODOS available, add a TODO first"),
                        String::from("\x1b[33m"),
                    );
                } else {
                    paint_cell(
                        String::from("Enter only 3 characters"),
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
                println!();
            }
        }
    }
}
