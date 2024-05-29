use serde::{Deserialize, Serialize};
use std::env::var_os;

use crate::db::DB;
use crate::todo::ToDoItem;

// Placeholder for when session data includes more than just todos.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionData {
    todo_items: Vec<ToDoItem>,
}

pub fn load_session(db: &DB) -> Option<Vec<ToDoItem>> {
    let todo_items: Vec<ToDoItem> = db.get_todos().unwrap();

    Some(todo_items)
}

fn path() -> String {
    let home_var = var_os("HOME").unwrap();
    let home_str = home_var.to_str().unwrap();

    format!("{home_str}/Documents/tpsession.json")
}
