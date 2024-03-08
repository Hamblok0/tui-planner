use serde::{Deserialize, Serialize};
use serde_json::{to_string, Result};
use std::env::var_os;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::Path;

use crate::todo::ToDoItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionData {
    todo_items: Vec<ToDoItem>,
}

pub fn save_session(data: &Vec<ToDoItem>) {
    let home_var = var_os("HOME").unwrap();
    let home_str = home_var.to_str().unwrap();
    let path = format!("{home_str}/Documents/tpsession.json");

    let session = SessionData {
        todo_items: data.clone(),
    };

    let json = to_string(&session).unwrap();

    if !Path::new(&path).exists() {
        
    }
}

pub fn load_session() -> Option<SessionData> {
    unimplemented!()
}

fn write(path: String) -> Result<()> {
    unimplemented!()
}

fn read(path: String) -> Result<()> {
    unimplemented!()
} 