use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::todo::ToDoItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoData {
    items: Vec<ToDoItem>
}

pub fn save_todos(data: &Vec<ToDoItem>) {

}