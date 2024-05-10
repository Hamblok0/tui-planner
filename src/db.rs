use rusqlite::{params, Connection, Result, Rows, Row};
use std::path::Path;
use std::env::var_os;
use fallible_iterator::FallibleIterator; 

use crate::todo::ToDoItem;

pub struct DB {
    db: Connection,
}

impl DB {
    pub fn new() -> Self {
        let home_var = var_os("HOME").unwrap();
        let home_str = home_var.to_str().unwrap();
        let path = format!("{home_str}/Documents/tpsession.db3");
        let exists = Path::new(&path).exists();
        let db = Connection::open(&path).unwrap();

        if !exists  {
            db.execute("CREATE TABLE todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                complete BOOLEAN NOT NULL CHECK (complete IN (0, 1))
            )", ()).unwrap();
        }
        // let data: Vec<ToDoItem> = load_session().unwrap();
        // let iter = data.iter();

        // for todo in iter {
        //     db.execute("INSERT INTO todos (title, description, complete) VALUES (?1, ?2, ?3)", (&todo.title, &todo.description, &todo.complete)).unwrap();
        // } 
        
        Self {
            db
        }
    }

    pub fn get_todos(&self) -> Result<Vec<ToDoItem>> {
        let mut stmt = self.db.prepare("SELECT * FROM todos")?;
        let mut rows: Rows = stmt.query([])?;

        let row_closure = |row: &Row| -> Result<ToDoItem> {
            let title = row.get(1)?;
            let description = row.get(2)?;
            let complete = row.get(3)?; 

            Ok(ToDoItem {
                title,
                description,
                complete
            })
        };

        rows.map(row_closure).collect()
    }
}
