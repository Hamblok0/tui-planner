use fallible_iterator::FallibleIterator;
use rusqlite::{params, Connection, Result, Row, Rows};
use std::env::var_os;
use std::path::Path;

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

        if !exists {
            db.execute(
                "CREATE TABLE todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                complete BOOLEAN NOT NULL CHECK (complete IN (0, 1))
            )",
                (),
            )
            .unwrap();
        }

        Self { db }
    }

    pub fn get_todos(&self) -> Result<Vec<ToDoItem>> {
        let mut stmt = self.db.prepare("SELECT * FROM todos")?;
        let rows: Rows = stmt.query([])?;

        let row_closure = |row: &Row| -> Result<ToDoItem> {
            let id = row.get(0)?;
            let title = row.get(1)?;
            let description = row.get(2)?;
            let complete = row.get(3)?;

            Ok(ToDoItem {
                id,
                title,
                description,
                complete,
            })
        };

        rows.map(row_closure).collect()
    }

    pub fn create_todo(&self, title: &str, description: &str) -> Result<usize> {
        self.db.execute(
            "INSERT INTO todos (title, description, complete) VALUES (?1, ?2, ?3)",
            (title, description, false),
        )?;

        Ok(self.db.last_insert_rowid() as usize)
    }

    pub fn delete_todo(&self, id: &usize) -> Result<()> {
        self.db
            .execute("DELETE FROM todos WHERE id = (?1)", params![id])?;

        Ok(())
    }

    pub fn edit_todo(&self, todo: &ToDoItem, old_todo: ToDoItem) -> Result<()> {
        let id = todo.id;

        let mut title = if todo.title != old_todo.title {
            let title = &todo.title;
            format!("title = '{title}'")
        } else {
            "".to_string()
        };

        let description = if todo.description != old_todo.description {
            let description = &todo.description;
            if !title.is_empty() {
                title.push_str(", ");
            }
            format!("description = '{description}'")
        } else {
            "".to_string()
        };

        let command = format!("UPDATE todos SET {title} {description} WHERE id = {id}");

        self.db.execute(&command, ()).unwrap();

        Ok(())
    }

    pub fn toggle_todo(&self, todo: &ToDoItem) -> Result<()> {
        let complete = todo.complete;
        let id = todo.id;
        let command = format!("UPDATE todos SET complete = {complete} WHERE id = {id}");

        self.db.execute(&command, ()).unwrap();

        Ok(())
    }
}
