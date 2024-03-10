use serde::{Deserialize, Serialize};
use serde_json::{to_writer, Result};
use std::env::var_os;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use crate::todo::ToDoItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionData {
    todo_items: Vec<ToDoItem>,
}

pub fn save_session(data: &Vec<ToDoItem>) {
    if data.len() == 0 {
        return
    }
    let home_var = var_os("HOME").unwrap();
    let home_str = home_var.to_str().unwrap();
    let path = format!("{home_str}/Documents/tpsession.json");

    let file = if !Path::new(&path).exists() {
        File::create(&path).unwrap()
    } else {
        OpenOptions::new().write(true).truncate(true).open(&path).unwrap()
    };

    let mut writer = BufWriter::new(file);

    to_writer(&mut writer, data).unwrap();
    writer.flush().unwrap();
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
