use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

trait Todo {
    fn add(&self);
    fn get_all(&self);
    fn delete(&self);
    fn complete(&mut self);
}

struct TodoItem {
    id: String,
    title: String,
    description: String,
    done: bool
}

impl Todo for TodoItem {
    fn add(&self) {
        // TODO: Add new todo
    }

    fn get_all(&self) {
        // TODO: Get all todos
    }

    fn delete(&self) {
        // TODO: Delete todo
    }

    fn complete(&mut self) {
        // TODO: Complete todo
        self.done = true
    }
}

fn generate_uuid() -> String {
    // TODO: Generate random uuid
    let lenght: u8 = 24;
    let random_string = 
}

fn main() {
    let tasks: Vec<Box<dyn Todo>> = Vec::new();
    println!("Hello, world!");
}
