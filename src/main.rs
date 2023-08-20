use prettytable::{color, Attr, Cell, Row, Table};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

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

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();

    loop {
        let mut input = String::new();
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("#")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::BackgroundColor(color::BLACK)),
            Cell::new("TITLE")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::BackgroundColor(color::BLACK)),
            Cell::new("DESCRIPTION")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::BackgroundColor(color::BLACK)),
            Cell::new("COMPLETED")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::GREEN))
                .with_style(Attr::BackgroundColor(color::BLACK)),
        ]));

        println!("What are you going to do?: \x1b[32m[a] Add\x1b[0m  \x1b[32m[l] To list\x1b[0m  \x1b[32m[d] Delete\x1b[0m  \x1b[32m[e] Edit\x1b[0m  \x1b[32m[q] Quit\x1b[0m");
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
                println!("description?: [String] [Optional]");
                println!();

                print!("\x1b[32mEnter the title > \x1b[0m");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).expect("Dont");

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
            "d" => println!("Delete TODO"),
            "e" => println!("Edit TODO"),
            "q" => break,
            _ => {
                let mut table_invalid_option = Table::new();
                table_invalid_option
                    .add_row(Row::new(vec![Cell::new("Not a valid option")
                        .with_style(Attr::ForegroundColor(color::RED))]));
                table_invalid_option.printstd();
            }
        }
    }
}
