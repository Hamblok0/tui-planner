use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct DB {
    db: Connection,
}

impl DB {
    pub fn new() -> Self {
        let home_var = var_os("HOME").unwrap();
        let home_str = home_var.to_str().unwrap();
        let path = format!("{home_str}/Documents/tpsession.db3");

        let db = Connection::open(path)?;

        if db.execute("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type="table" AND name = "tpsession")", params)

    }
}
