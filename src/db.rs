use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::env::var_os;

use crate::todo::ToDoItem;

pub struct DB {
    db: Connection,
}

impl DB {
    pub fn new() -> Self {
        let home_var = var_os("HOME").unwrap();
        let home_str = home_var.to_str().unwrap();
        let path = format!("{home_str}/Documents/tpsession.db3");

        let db = Connection::open(&path).unwrap();

        if Path::new(&path).exists() {
            db.execute("CREATE TABLE todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                complete BOOLEAN NOT NULL
            )", ()).unwrap();
        }

        Self {
            db
        }
    }

    // pub fn write_todos(&self, data: &Vec<ToDoItem>) -> Result<()> {
    //     self.db.execute("", ())?;
    // }
}
