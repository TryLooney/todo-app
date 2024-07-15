// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io::prelude::*;
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
}

fn gen_todo(content: String) -> Todo {
    let id = Ulid::new();
    Todo {
        content,
        id: id.to_string(),
    }
}

fn save_todos(todos: &Todos) -> Result<(), Box<dyn std::error::Error>> {
    let j = serde_json::to_string(&todos)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todos.json")?;
    file.write_all(j.as_bytes())?;
    Ok(())
}

fn load_todos() -> Result<Todos, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).open("todos.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let todos: Todos = serde_json::from_str(&contents)?;
    Ok(todos)
}

#[tauri::command]
fn get_todos() -> Todos {
    let mut todos = match load_todos() {
        Ok(todos) => todos,
        Err(_) => Todos { todos: Vec::new() },
    };

    todos.todos.sort_by(|a, b| a.id.cmp(&b.id));
    todos
}

#[tauri::command]
fn create_todo(content: String) -> Todos {
    let mut todos = match load_todos() {
        Ok(todos) => todos,
        Err(_) => Todos { todos: Vec::new() },
    };
    todos.todos.sort_by(|a, b| a.id.cmp(&b.id));

    let todo = gen_todo(content);
    todos.todos.push(todo);

    // Save the updated list of todos
    if let Err(e) = save_todos(&todos) {
        eprintln!("Failed to save todos: {}", e);
    }

    todos
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_todos, create_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
