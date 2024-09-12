use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::usize;

#[derive(Serialize, Deserialize)]
struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        TodoItem {
            name,
            completed: ' ', // Default incomplete state
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn add_task(&mut self, name: String) {
        self.list.push(TodoItem::new(name));
    }

    fn print(&self) {
        for (iter, item) in self.list.iter().enumerate() {
            println!("{} - [{}] - {}", iter, item.completed, item.name);
        }
    }
    fn mark_completed(&mut self, index: usize) {
        self.list[index].completed = 'x'
    }
    fn delete_task(&mut self, index: usize) {
        self.list.remove(index);
    }
    fn save(&self) {
        let serialized = serde_json::to_string(&self).expect("Failed to serialize todo list");
        fs::write("todo_list.json", serialized).expect("Failed to write to file");
    }

    fn load() -> TodoList {
        if let Ok(data) = fs::read_to_string("todo_list.json") {
            serde_json::from_str(&data).unwrap_or_else(|_| TodoList::new())
        } else {
            TodoList::new()
        }
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
    Delete(usize),
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse().expect("Error converting to an index")),
        "delete" => Command::Delete(args[2].parse().expect("Failed to convert to an index")),
        _ => panic!("Provide an accepted commmand"),
    };

    let mut todo_list = TodoList::load();

    match command {
        Command::Get => todo_list.print(),

        Command::Add(task) => {
            todo_list.add_task(task);
            todo_list.print();
            todo_list.save();
        }

        Command::Done(index) => {
            todo_list.mark_completed(index);
            todo_list.print();
            todo_list.save()
        }
        Command::Delete(index) => {
            todo_list.delete_task(index);
            todo_list.print();
            todo_list.save()
        }
    }
}
