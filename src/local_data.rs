use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::env::var_os;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::todo::ToDoItem;
use crate::db::DB;

// Placeholder for when session data includes more than just todos.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionData {
    todo_items: Vec<ToDoItem>,
}

pub fn save_session(data: &Vec<ToDoItem>) {
    let path = path();
    let file = if !Path::new(&path).exists() {
        File::create(path).unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .unwrap()
    };

    let mut writer = BufWriter::new(file);

    to_writer(&mut writer, &data).unwrap();
    writer.flush().unwrap();
}

pub fn load_session(db: &DB) -> Option<Vec<ToDoItem>> {
    let todo_items: Vec<ToDoItem> = db.get_todos().unwrap();

    return Some(todo_items);
}

fn path() -> String {
    let home_var = var_os("HOME").unwrap();
    let home_str = home_var.to_str().unwrap();
    return format!("{home_str}/Documents/tpsession.json");
}
